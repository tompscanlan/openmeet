import { hash } from "bcrypt";
import { v4 as uuidv4 } from "uuid";
import cassandraClient from "../utils/cassandra";

export default defineEventHandler(async (event) => {
  const body = await readBody(event);
  const { username, email, password } = body;

  const userId = uuidv4();
  const passwordHash = await hash(password, 10);

  const query =
    "INSERT INTO users (user_id, username, email, password_hash) VALUES (?, ?, ?, ?)";
  const params = [userId, username, email, passwordHash];

  try {
    await cassandraClient.execute(query, params, { prepare: true });
    return { message: `User ${email} registered successfully` };
  } catch (error) {
    throw createError({
      statusCode: 500,
      statusMessage: "Failed to register user",
    });
  }
});
