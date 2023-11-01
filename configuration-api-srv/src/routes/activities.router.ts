
import { FastifyInstance } from 'fastify';
import * as controllers from '../controllers';

async function activitiesRouter(fastify: FastifyInstance) {
    fastify.route({
        method: 'GET',
        url: '/',
        handler: controllers.getActivities
    });
    fastify.route({
        method: 'GET',
        url: '/:id',
        handler: controllers.getActivityById
    });
    fastify.route({
        method: 'GET',
        url: '/code/:code',
        handler: controllers.getActivityByCode
    });

    fastify.route({
        method: 'POST',
        url: '/',
        handler: controllers.createActivity
    });
    fastify.route({
        method: 'PUT',
        url: '/:id',
        handler: controllers.updateActivity
    });
    fastify.route({
        method: 'DELETE',
        url: '/:id',
        handler: controllers.deleteActivity
    });
    fastify.route({
        method: 'PUT',
        url: '/:activityId/segments/:segmentId',
        handler: controllers.assignActivityToSegment
    });
    fastify.route({
        method: 'DELETE',
        url: '/:activityId/segments/:segmentId',
        handler: controllers.unassignActivityFromSegment
    });

}

export default activitiesRouter;