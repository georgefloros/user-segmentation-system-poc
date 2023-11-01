import { FastifyReply, FastifyRequest } from "fastify";
import { prisma } from "../utils";
import { Prisma } from '@prisma/client';


export const createUser = async (request: FastifyRequest<{ Body: Prisma.UserCreateInput; }>, reply: FastifyReply) => {

    try {
        const { body } = request;
        const user = await prisma.user.create({ data: body });
        reply.status(201).send({ data: user });
    } catch (error) {
        reply.log.error(error);
        reply.status(500).send();
    }
};
export const updateUser = async (request: FastifyRequest<{ Params: { id: number; }, Body: Prisma.UserCreateInput; }>, reply: FastifyReply) => {

    try {
        const { body } = request;
        const { id } = request.params;
        const user = await prisma.user.update({ where: { id: +id }, data: body });
        reply.status(200).send({ data: user });
    } catch (error) {
        reply.log.error(error);
        reply.status(500).send();
    }
};
export const getUsers = async (request: FastifyRequest, reply: FastifyReply) => {
    try {

        const users = await prisma.user.findMany({ include: { segments: true } });
        reply.status(200).send({ data: users });
    } catch (error) {
        reply.log.error(error);
        reply.status(500).send();
    }
};
export const getUserById = async (request: FastifyRequest<{ Params: { id: number; }; }>, reply: FastifyReply) => {
    try {
        const { id } = request.params;
        const user = await prisma.user.findUnique({ where: { id: +id }, include: { segments: true } });
        reply.status(200).send({ data: user });
    } catch (error) {
        reply.log.error(error);
        reply.status(500).send();
    }
};
export const updateSegments = async (request: FastifyRequest<{ Params: { id: number; }, Body: { segments: number[]; }; }>, reply: FastifyReply) => {
    try {
        const { id } = request.params;
        const { segments } = request.body;
        const user = await prisma.user.update({
            where: { id: +id },
            data: {
                segments: {
                    deleteMany: {},
                    create: segments.map(segmentId => ({ segment: { connect: { id: +segmentId } } }))
                }
            }
        });
        reply.status(200).send({ data: user });
    } catch (error) {
        reply.log.error(error);
        reply.status(500).send();
    }
};
export const getUsersBySegmentTag = async (request: FastifyRequest<{ Params: { tag: string; }; }>, reply: FastifyReply) => {
    try {
        const { tag } = request.params;
        const user = await prisma.user.findMany({
            where: {
                OR: [
                    {
                        segments: {
                            some: { segment: { tag } }
                        }
                    },
                ]
            }
        });
        reply.status(200).send({ data: user });
    } catch (error) {
        reply.log.error(error);
        reply.status(500).send();
    }
};

