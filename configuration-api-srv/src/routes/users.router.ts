import { FastifyInstance } from 'fastify';
import * as controllers from '../controllers';

async function usersRouter(fastify: FastifyInstance) {

    fastify.route({
        method: 'GET',
        url: '/',
        handler: controllers.getUsers
    });
    fastify.route({
        method: 'POST',
        url: '/',
        handler: controllers.createUser
    });
    fastify.route({
        method: 'PUT',
        url: '/:id',
        handler: controllers.updateUser
    });

    fastify.route({
        method: 'GET',
        url: '/:id',
        handler: controllers.getUserById
    });

    fastify.route({
        method: 'GET',
        url: '/client-ref-id/:id',
        handler: controllers.updateUserByClientRefId
    });
    fastify.route({
        method: 'GET',
        url: '/segment/:tag',
        handler: controllers.getUsersBySegmentTag
    });

}
export default usersRouter;