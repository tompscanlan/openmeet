import { compare } from "bcrypt";
import jwt from "jsonwebtoken";
import cassandraClient from "../utils/cassandra";
import { defineEventHandler, readBody, createError } from "h3";

const JWT_SECRET = process.env.JWT_SECRET || "your_jwt_secret";

export default defineEventHandler(async (event) => {
  const body = await readBody(event);
  const { email, password } = body;

  const query = "SELECT * FROM users WHERE email = ?";
  const params = [email];
  let result;
  let user;
  try {
    try {
      result = await cassandraClient.execute(query, params, { prepare: true });
      console.error("Result:", result);
    } catch (error) {
      console.error("Error query:", error);
      throw error;
    }
    try {
      user = result.rows[0];
    } catch (error) {
      console.error("Error user:", error);
      throw error;
    }

    if (!user || !(await compare(password, user.password_hash))) {
      throw createError({
        statusCode: 401,
        statusMessage: "Invalid credentials",
      });
    }

    const token = jwt.sign(
      { userId: user.user_id, email: user.email },
      JWT_SECRET,
    );
    return { success: true, message: "Login successful", token };
  } catch (error) {
    throw createError({
      statusCode: 500,
      statusMessage: "Failed to login",
    });
  }
});
