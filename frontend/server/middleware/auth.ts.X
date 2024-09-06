import jwt from 'jsonwebtoken';

const JWT_SECRET = process.env.JWT_SECRET || 'your_jwt_secret';

export default defineEventHandler((event) => {
  const token = event.node.req.headers.authorization?.replace('Bearer ', '');
  
  if (!token) {
    throw createError({
      statusCode: 401,
      statusMessage: 'Authentication required',
    });
  }

  try {
    const decoded = jwt.verify(token, JWT_SECRET);
    event.context.auth = decoded;
  } catch (error) {
    throw createError({
      statusCode: 401,
      statusMessage: 'Invalid token',
    });
  }
});