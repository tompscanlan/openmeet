import { describe, it, expect, beforeEach, vi } from "vitest";
import { createError } from "h3";
import jwt from "jsonwebtoken";
import { compare } from "bcrypt";
import cassandraClient from "../utils/cassandra";
import { readBody } from "h3";

// Mock the necessary dependencies
vi.mock("h3", async () => {
  const actual = await vi.importActual("h3");

  return {
    ...actual,
    readBody: vi.fn(),
    createError: vi.fn(),
  };
});
vi.mock("bcrypt");
vi.mock("jsonwebtoken");
vi.mock("../utils/cassandra");

// Import the handler after mocking dependencies
const loginHandler = (await import("./login")).default;

describe("Login Handler", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("should return a token on successful login", async () => {
    const mockUser = {
      user_id: "123",
      email: "test@example.com",
      password_hash: "hashedPassword",
    };
    const mockToken = "mockJWTToken";

    vi.mocked(readBody).mockResolvedValue({
      email: "test@example.com",
      password: "password123",
    });
    vi.mocked(cassandraClient.execute).mockResolvedValue({
      rows: [mockUser],
    } as any);
    vi.mocked(compare).mockResolvedValue(true as never);
    vi.mocked(jwt.sign).mockImplementation(
      (payload, secret, options, callback) => {
        console.log("JWT sign called with:", { payload, secret, options });
        if (callback) {
          callback(null, mockToken);
        } else {
          console.error("JWT sign callback is undefined");
          return mockToken;
        }
      },
    );

    try {
      const result = await loginHandler({} as any);
      console.error(result);
      expect(result).toEqual({
        success: true,
        message: "Login successful",
        token: mockToken,
      });
    } catch (error) {
      console.error("Error in login handler:", error);
      throw error;
    }
  });

  it("should throw an error for invalid credentials", async () => {
    vi.mocked(readBody).mockResolvedValue({
      email: "test@example.com",
      password: "wrongpassword",
    });
    vi.mocked(cassandraClient.execute).mockResolvedValue({ rows: [] } as any);

    const mockError = createError({
      statusCode: 401,
      statusMessage: "Invalid credentials",
    });
    vi.mocked(createError).mockReturnValue(mockError);

    await expect(loginHandler({} as any)).rejects.toThrow(
      "Invalid credentials",
    );
  });

  it("should throw an error when database query fails", async () => {
    vi.mocked(readBody).mockResolvedValue({
      email: "test@example.com",
      password: "password123",
    });
    vi.mocked(cassandraClient.execute).mockRejectedValue(
      new Error("Database error"),
    );

    const mockError = createError({
      statusCode: 500,
      statusMessage: "Failed to login",
    });
    vi.mocked(createError).mockReturnValue(mockError);

    await expect(loginHandler({} as any)).rejects.toThrow("Failed to login");
  });
});
