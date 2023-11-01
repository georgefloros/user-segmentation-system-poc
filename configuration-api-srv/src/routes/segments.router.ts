import { FastifyInstance } from 'fastify';
import * as controllers from '../controllers';

async function segmentsRouter(fastify: FastifyInstance) {
    fastify.route({
        method: 'POST',
        url: '/',
        handler: controllers.createSegment
    });
    fastify.route({
        method: 'PUT',
        url: '/:id',
        handler: controllers.updateSegment
    });
    fastify.route({
        method: 'DELETE',
        url: '/:id',
        handler: controllers.deleteSegment
    });
    fastify.route({
        method: 'PUT',
        url: '/:segmentId/activities/:activityId',
        handler: controllers.assignSegmentToActivity
    });
    fastify.route({
        method: 'DELETE',
        url: '/:segmentId/activities/:activityId',
        handler: controllers.unassignSegmentFromActivity
    });
    fastify.route({
        method: 'GET',
        url: '/',
        handler: controllers.getSegments
    });
    fastify.route({
        method: 'GET',
        url: '/:id',
        handler: controllers.getSegmentById
    });
    fastify.route({
        method: 'GET',
        url: '/generics-and-by/:code',
        handler: controllers.getGenericsAndByActivityCode
    });
}

export default segmentsRouter;