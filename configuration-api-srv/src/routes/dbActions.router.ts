import { FastifyInstance } from 'fastify';
import * as controllers from '../controllers';

async function dbActions(fastify: FastifyInstance) {
    fastify.route({
        method: 'POST',
        url: '/seed/analytics',
        handler: controllers.createAnalyticsData
    });
    fastify.route({
        method: 'POST',
        url: '/seed/data',
        handler: controllers.createData
    });
}
export default dbActions;
