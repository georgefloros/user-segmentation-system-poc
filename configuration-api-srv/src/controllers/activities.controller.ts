import { FastifyReply, FastifyRequest } from "fastify";
import { prisma } from "../utils";
import { Prisma } from '@prisma/client';
export const getActivities = async (request: FastifyRequest, reply: FastifyReply) => {
    try {

        const activities = await prisma.activity.findMany();
        reply.status(200).send({ data: activities });
    } catch (error) {
        reply.log.error(error);
        reply.status(500).send();
    }
};
export const getActivityById = async (request: FastifyRequest<{ Params: { id: number; }; }>, reply: FastifyReply) => {
    try {
        const { id } = request.params;
        const activity = await prisma.activity.findUnique({ where: { id: +id } });
        reply.status(200).send({ data: activity });
    } catch (error) {
        reply.log.error(error);
        reply.status(500).send();
    }
};
export const getActivityByCode = async (request: FastifyRequest<{ Params: { code: string; }; }>, reply: FastifyReply) => {
    try {
        const { code } = request.params;
        const activity = await prisma.activity.findFirst({
            where: { code: code },
        });
        reply.status(200).send({ data: activity });
    } catch (error) {
        reply.log.error(error);
        reply.status(500).send();
    }
};

export const createActivity = async (request: FastifyRequest<{ Body: Prisma.ActivityCreateInput; }>, reply: FastifyReply) => {
    try {
        const { body } = request;
        const activity = await prisma.activity.create({ data: body });
        reply.status(201).send({ data: activity });
    } catch (error) {
        reply.log.error(error);
        reply.status(500).send();
    }
};

export const updateActivity = async (request: FastifyRequest<{ Params: { id: number; }, Body: Prisma.ActivityCreateInput; }>, reply: FastifyReply) => {
    try {

        const { id } = request.params;
        const { body } = request;
        const activity = await prisma.activity.update({ where: { id: +id }, data: body });
        reply.status(200).send({ data: activity });
    } catch (error) {
        reply.log.error(error);
        reply.status(500).send();
    }
};

export const deleteActivity = async (request: FastifyRequest<{ Params: { id: number; }; }>, reply: FastifyReply) => {
    try {

        const { id } = request.params;
        const activity = await prisma.activity.delete({ where: { id: +id } });
        reply.status(200).send({ data: true });
    } catch (error) {
        reply.log.error(error);
        reply.status(500).send();
    }
};
export const assignActivityToSegment = async (request: FastifyRequest<{ Params: { activityId: number, segmentId: number; }; }>, reply: FastifyReply) => {
    try {

        const { activityId, segmentId } = request.params;
        await prisma.segmentActivities.create({ data: { activityId: +activityId, segmentId: +segmentId } });
        reply.status(200).send({ data: true });
    } catch (error) {
        reply.log.error(error);
        reply.status(500).send();
    }
};
export const unassignActivityFromSegment = async (request: FastifyRequest<{ Params: { activityId: number, segmentId: number; }; }>, reply: FastifyReply) => {
    try {

        const { activityId, segmentId } = request.params;
        await prisma.segmentActivities.delete({ where: { segmentId_activityId: { segmentId: +segmentId, activityId: +activityId } } });
        reply.status(200).send({ data: true });
    } catch (error) {
        reply.log.error(error);
        reply.status(500).send();
    }
};