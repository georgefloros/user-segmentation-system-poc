
import { prisma } from './utils';
import userRouter from './routes/users.router';
import activitiesRouter from './routes/activities.router';
import segmentsRouter from './routes/segments.router';
import dbActions from './routes/dbActions.router';
import fastify from 'fastify';
const startServer = async () => {
  try {
    const server = fastify({
      logger: true, ignoreDuplicateSlashes: true
    });
    const apiVersion = process.env.API_VERSION || 'v1';
    server.register(userRouter, { prefix: `/api/${apiVersion}/users` });
    server.register(activitiesRouter, { prefix: `/api/${apiVersion}/activities` });
    server.register(segmentsRouter, { prefix: `/api/${apiVersion}/segments` });
    server.register(dbActions, { prefix: `/api/${apiVersion}/db-actions` });
    server.get('/health-check', async (request, reply) => {
      try {
        await prisma.$queryRaw`SELECT 1`;
        reply.status(200).send();
      } catch (e) {
        reply.status(500).send();
      }
    });
    const port = Number(process.env.API_PORT) || 3000;
    await server.listen({ port });
    console.log(`🚀 Server ready at: http://localhost:${port}`);
  }
  catch (err) {
    exit(err);
  }
};

function exit(e: any) {
  console.error(e);
  prisma.$disconnect();
  process.exit(1);
}
process.on('unhandledRejection', exit);
startServer();
