import { FastifyReply, FastifyRequest } from "fastify";
import { prisma } from "../utils";
import { Prisma } from '@prisma/client';

export const createSegment = async (request: FastifyRequest<{ Body: Prisma.SegmentCreateInput; }>, reply: FastifyReply) => {
    try {
        const { body } = request;
        const segment = await prisma.segment.create({ data: body });
        reply.status(201).send({ data: segment });
    } catch (error) {
        reply.log.error(error);
        reply.status(500).send();
    }
};
export const getSegments = async (request: FastifyRequest, reply: FastifyReply) => {
    try {
        const segments = await prisma.segment.findMany();
        reply.status(200).send({ data: segments });
    } catch (error) {
        reply.log.error(error);
        reply.status(500).send();
    }
};
export const getSegmentById = async (request: FastifyRequest<{ Params: { id: number; }; }>, reply: FastifyReply) => {
    try {
        const { id } = request.params;
        const segments = await prisma.segment.findUnique({ where: { id: +id } });
        reply.status(200).send({ data: segments });
    } catch (error) {
        reply.log.error(error);
        reply.status(500).send();
    }
};
export const updateSegment = async (request: FastifyRequest<{ Params: { id: number; }, Body: Prisma.SegmentCreateInput; }>, reply: FastifyReply) => {
    try {
        const { id } = request.params;
        const { body } = request;
        const segment = await prisma.segment.update({ where: { id: +id }, data: body });
        reply.status(200).send({ data: segment });
    } catch (error) {
        reply.log.error(error);
        reply.status(500).send();
    }

};

export const deleteSegment = async (request: FastifyRequest<{ Params: { id: number; }; }>, reply: FastifyReply) => {
    try {
        const { id } = request.params;
        await prisma.segment.delete({ where: { id: +id } });
        reply.status(200).send({ data: true });
    } catch (error) {
        reply.log.error(error);
        reply.status(500).send();
    }
};

export const assignSegmentToActivity = async (request: FastifyRequest<{ Params: { activityId: number, segmentId: number; }; }>, reply: FastifyReply) => {
    try {

        const { activityId, segmentId } = request.params;
        await prisma.segmentActivities.create({ data: { activityId: +activityId, segmentId: +segmentId } });
        reply.status(200).send({ data: true });
    } catch (error) {
        reply.log.error(error);
        reply.status(500).send();
    }
};
export const unassignSegmentFromActivity = async (request: FastifyRequest<{ Params: { activityId: number, segmentId: number; }; }>, reply: FastifyReply) => {
    try {

        const { activityId, segmentId } = request.params;
        await prisma.segmentActivities.delete({ where: { segmentId_activityId: { segmentId: +segmentId, activityId: +activityId } } });
        reply.status(200).send({ data: true });
    } catch (error) {
        reply.log.error(error);
        reply.status(500).send();
    }
};
export const getGenericsAndByActivityCode = async (request: FastifyRequest<{ Params: { code: string; }; }>, reply: FastifyReply) => {
    try {
        const { code } = request.params;
        const segments = await prisma.segment.findMany(
            {
                where: {
                    OR: [
                        { isGeneric: true, },
                        {
                            activities: {
                                some: { activity: { code: code } }
                            }
                        },
                    ]
                }
            }
        );
        reply.status(200).send({ data: segments });
    } catch (error) {
        reply.log.error(error);
        reply.status(500).send();
    }
};