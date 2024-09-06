import { hash, compare } from "bcrypt";
import { v4 as uuidv4 } from "uuid";
import jwt from "jsonwebtoken";
import cassandraClient from "../utils/cassandra";

const JWT_SECRET = process.env.JWT_SECRET || "your_jwt_secret";

interface User {
  user_id: string;
  username: string;
  email: string;
  password_hash?: string;
  description?: string;
  interests: string[];
  created_at?: number;
  updated_at?: number;
  last_login?: number;
}

export async function createUser(user: User): Promise<User> {
  const userId = uuidv4();

  if (user.password_hash) {
    user.password_hash = await hash(user.password_hash, 10);
  } else {
    throw new Error("Password is required");
  }

  const now = Date.now();
  const query =
    "INSERT INTO openmeet.users (user_id, username, email, password_hash, created_at, updated_at, last_login) VALUES (?, ?, ?, ?, ?, ?, ?)";
  const params = [
    userId,
    user.username,
    user.email,
    user.password_hash,
    now,
    0,
    0,
  ];

  await cassandraClient.execute(query, params, { prepare: true });

  // Insert email index
  const emailIndexQuery =
    "INSERT INTO openmeet.email_index (email, user_id) VALUES (?, ?)";
  await cassandraClient.execute(emailIndexQuery, [user.email, userId], {
    prepare: true,
  });

  return {
    ...user,
    user_id: userId,
    created_at: now,
    updated_at: now,
    last_login: now,
  };
}

export async function getUserById(userId: string): Promise<User | null> {
  const query = "SELECT * FROM openmeet.users WHERE user_id = ?";
  const result = await cassandraClient.execute(query, [userId], {
    prepare: true,
  });

  if (result.rows.length === 0) {
    return null;
  }

  const row = result.first();
  return {
    user_id: row.user_id.toString(),
    username: row.username,
    email: row.email,
    password_hash: row.password_hash,
    description: row.description,
    interests: row.interests || [],
    created_at: row.created_at,
    updated_at: row.updated_at,
    last_login: row.last_login,
  };
}

export async function getUserByEmail(email: string): Promise<User | null> {
  const query = "SELECT * FROM openmeet.users WHERE email = ?";
  const result = await cassandraClient.execute(query, [email], {
    prepare: true,
  });

  if (result.rows.length === 0) {
    return null;
  }

  const row = result.first();
  return {
    user_id: row.user_id.toString(),
    username: row.username,
    email: row.email,
    password_hash: row.password_hash,
    description: row.description,
    interests: row.interests || [],
    created_at: row.created_at,
    updated_at: row.updated_at,
    last_login: row.last_login,
  };
}

export async function deleteUser(userId: string, email: string): Promise<void> {
  const deleteUserQuery = "DELETE FROM openmeet.users WHERE user_id = ?";
  await cassandraClient.execute(deleteUserQuery, [userId], { prepare: true });

  const deleteEmailIndexQuery =
    "DELETE FROM openmeet.email_index WHERE email = ?";
  await cassandraClient.execute(deleteEmailIndexQuery, [email], {
    prepare: true,
  });
}

export default defineEventHandler(async (event) => {
  const { method, url } = event.node.req;

  if (method === "GET" && url?.startsWith("/api/users/")) {
    const userId = url.split("/").pop();
    const user = await getUserById(userId!);
    return { user };
  }

  if (method === "DELETE" && url?.startsWith("/api/users/")) {
    const userId = url.split("/").pop();
    const body = await readBody(event);
    try {
      await deleteUser(userId!, body.email);
      return { success: true, message: "User deleted successfully" };
    } catch (error) {
      throw createError({
        statusCode: 500,
        statusMessage: "Failed to delete user",
      });
    }
  }
});
