import { FastifyReply, FastifyRequest } from "fastify";
import { faker } from '@faker-js/faker';

import { prisma } from "../utils";
import { Client } from 'databend-driver';
import { Prisma } from '@prisma/client';


function getActivityData(): Array<Prisma.ActivityCreateInput> {
    return [
        {

            title: 'page element click',
            description: 'User visited a page and clicked an element',
            code: 'page_element_clicked',
        },
        {
            title: 'page view',
            description: 'User visited a page',
            code: 'page_viewed',
        },
        {
            title: 'product purchased',
            description: 'User purchased a product',
            code: 'product_purchased'
        }
    ];

};
function getSegmentData(): Array<Prisma.SegmentCreateInput> {
    return [
        {
            title: 'active visitors',
            description: 'Users who have been browsing the site for the last 30 minutes',
            tag: "active_visitor",
            whereStatement: "",
            isGeneric: true
        },
        {
            title: 'area no buyers',
            description: "users from California who haven't made purchase for the last 30 days",
            tag: "area_no_buyers",
            whereStatement: "",
            isGeneric: true,
        },
        {
            title: 'mobile users with no activity',
            description: "mobile users who visited a page but haven't been active for the last 7 days",
            tag: "mobile_no_recent_activity",
            whereStatement: "",
            isGeneric: false,
        }
    ];
}
function getSegmentActivityData(relations: { activityId: number, segmentId: number; }[]): Array<Prisma.SegmentActivitiesCreateInput> {
    return relations.map((r) => ({
        segment: { connect: { id: r.segmentId } },
        activity: { connect: { id: r.activityId } }
    }));
}

export const createData = async (request: FastifyRequest, reply: FastifyReply) => {
    try {
        reply.log.info("Creating data");
        await prisma.segmentActivities.deleteMany();
        await prisma.usersInSegments.deleteMany();
        await prisma.user.deleteMany();
        await prisma.activity.deleteMany();
        await prisma.segment.deleteMany();
        for (const i of Array.from({ length: 100 })) {
            const user = await prisma.user.create({
                data: {
                    name: faker.person.firstName(),
                    email: faker.internet.email(),
                    clientRefId: faker.string.uuid(),
                },
            });
            reply.log.info(`Created user with id: ${user.id}`);
        }
        const activityData = getActivityData();
        for (const a of activityData) {
            const activity = await prisma.activity.create({
                data: a,
            });
            reply.log.info(`Created activity with id: ${activity.id}`);
        }

        const segmentData = getSegmentData();
        for (const s of segmentData) {
            const segment = await prisma.segment.create({
                data: s,
            });
            reply.log.info(`Created segment with id: ${segment.id}`);
        }
        reply.log.info(`Creating relations`);
        //create relations
        const activity = await prisma.activity.findFirst({ where: { code: 'page_viewed' } });
        const segment = await prisma.segment.findFirst({ where: { tag: 'mobile_no_recent_activity' } });
        if (activity && segment) {
            const segmentActivitiesData = getSegmentActivityData([{ activityId: activity.id, segmentId: segment.id }]);
            for (const sa of segmentActivitiesData) {
                const segmentActivity = await prisma.segmentActivities.create({
                    data: sa,
                });
                reply.log.info(`Created segment:${segmentActivity.segmentId} - activity:${segmentActivity.segmentId} relation`);
            }
        }
        reply.log.info(`Seeding finished.`);
        reply.status(200).send({ data: true });
    } catch (error) {
        reply.log.error(error);
        reply.status(500).send();
    }

};


export const createAnalyticsData = async (request: FastifyRequest, reply:
    FastifyReply) => {
    try {
        const users = await prisma.user.findMany({ skip: 0, take: 100 });
        if (!users.length || users.length === 0) {
            reply.status(200).send({ data: false });
            return;
        }
        reply.log.info(`Creating analytics data for ${users.length} users`);
        reply.log.info("Creating analytics data");
        const dsn = process.env.DATABEND_CONNECTION_STRING
            ? process.env.DATABEND_CONNECTION_STRING
            : "databend://admin:1234Admin!@localhost:8000/default?sslmode=disable";
        const client = new Client(dsn);
        const conn = await client.getConn();
        reply.log.info`Connected to Databend`;

        let r = await conn.exec(`CREATE DATABASE IF NOT EXISTS user_segment_analytics`);
        reply.log.info(r);
        r = await conn.exec(`DROP TABLE user_segment_analytics.events`);

        r = await conn.exec(`CREATE TABLE IF NOT EXISTS user_segment_analytics.events (
            id VARCHAR,
            user_id INT32,
            client_ref_id VARCHAR,
            id_type VARCHAR,
            region VARCHAR,
            device_type VARCHAR,
            country VARCHAR,
            payload_activity_type VARCHAR,
            payload_url VARCHAR,
            payload_order_total DECIMAL(10, 2),
            payload_order_id VARCHAR,
            payload_element_id VARCHAR,
            created_at TIMESTAMP,
            processed_at TIMESTAMP
        );`);
        reply.log.info(r);
        const deviceTypes = ['mobile', 'desktop'];
        const usaStates = ["California", "Texas", "Florida", "New York", "Pennsylvania", "Illinois", "Ohio", "Georgia", "North Carolina", "Michigan", "New Jersey", "Virginia", "Washington", "Arizona", "Massachusetts", "Tennessee", "Indiana", "Missouri", "Maryland", "Wisconsin", "Colorado", "Minnesota", "South Carolina", "Alabama", "Louisiana", "Kentucky", "Oregon", "Oklahoma", "Connecticut", "Utah", "Iowa", "Nevada", "Arkansas", "Mississippi", "Kansas", "New Mexico", "Nebraska", "West Virginia", "Idaho", "Hawaii", "New Hampshire", "Maine", "Montana", "Rhode Island", "Delaware", "South Dakota", "North Dakota", "Alaska", "District of Columbia", "Vermont", "Wyoming"];
        const countries = ['US', 'GR', 'DE', 'FR', 'UK'];
        const activityTypes = ['page_viewed', 'page_element_clicked', 'product_purchased'];
        for (const i of Array.from({ length: 1000 })) {

            const user = faker.helpers.arrayElement(users);

            const id = faker.string.uuid();
            const idType = faker.helpers.arrayElement(['email', 'username', "cookie"]);
            const region = faker.helpers.arrayElement(usaStates);
            const deviceType = faker.helpers.arrayElement(deviceTypes);
            const country = faker.helpers.arrayElement(countries);
            const payloadActivityType = faker.helpers.arrayElement(activityTypes);
            const payloadUrl = faker.internet.url();
            const payloadOrderTotal = faker.helpers.rangeToNumber({ min: 1, max: 10000 });
            const payloadOrderId = faker.string.uuid();
            const payloadElementId = faker.string.uuid();

            const date = faker.date.between({ from: '2023-09-01', to: '2023-10-31' }).toISOString();
            r = await conn.exec(`INSERT INTO user_segment_analytics.events VALUES (
                '${id}',
                ${user.id},
                '${user.clientRefId}',
                '${faker.string.uuid()}',
                '${idType}',
                '${region}',
                '${deviceType}',
                '${country}',
                '${payloadActivityType}',
                '${payloadUrl}',
                ${payloadOrderTotal},
                '${payloadOrderId}',
                '${payloadElementId}',
                '${date}',
                '${date}'
            )`);
            reply.log.info(`Inserted event at index ${i} with id: ${id}`);
        }
    }
    catch (error) {
        reply.log.error(error);
        reply.status(500).send();
    }
}


/*
export const createCaches = async (request: FastifyRequest, reply: FastifyReply) => {

    try {

        let r = await prisma.$executeRawUnsafe('CREATE CACHE FROM SELECT "public"."Segment"."id", "public"."Segment"."title", "public"."Segment"."description", "public"."Segment"."tag", "public"."Segment"."whereStatement", "public"."Segment"."isGeneric", "public"."Segment"."createdAt", "public"."Segment"."updatedAt" FROM "public"."Segment" WHERE 1=1 OFFSET $1');
        reply.log.info(r);

        // r = await prisma.$queryRaw`CREATE CACHE FROM SELECT "public"."Segment"."id", "public"."Segment"."title", "public"."Segment"."description", "public"."Segment"."tag", "public"."Segment"."whereStatement", "public"."Segment"."isGeneric", "public"."Segment"."createdAt", "public"."Segment"."updatedAt" FROM "public"."Segment" WHERE ("public"."Segment"."id" = $1 AND 1=1) LIMIT $2 OFFSET $3`;


        // r = await prisma.$queryRaw`CREATE CACHE FROM SELECT "public"."Activity"."id", "public"."Activity"."title", "public"."Activity"."description", "public"."Activity"."code", "public"."Activity"."createdAt", "public"."Activity"."updatedAt" FROM "public"."Activity" WHERE 1=1 OFFSET $1`;


        // r = await prisma.$queryRaw`CREATE CACHE FROM SELECT "public"."Activity"."id", "public"."Activity"."title", "public"."Activity"."description", "public"."Activity"."code", "public"."Activity"."createdAt", "public"."Activity"."updatedAt" FROM "public"."Activity" WHERE ("public"."Activity"."id" = $1 AND 1=1) LIMIT $2 OFFSET $3`;

        // r = await prisma.$queryRaw`CREATE CACHE FROM SELECT "public"."User"."id", "public"."User"."name", "public"."User"."email", "public"."User"."createdAt", "public"."User"."updatedAt" FROM "public"."User" WHERE ("public"."User"."id" = $1 AND 1=1) LIMIT $2 OFFSET $3`;

        // r = await prisma.$queryRaw`CREATE CACHE FROM SELECT "public"."UsersInSegments"."userId", "public"."UsersInSegments"."segmentId", "public"."UsersInSegments"."createdAt", "public"."UsersInSegments"."updatedAt" FROM "public"."UsersInSegments" WHERE "public"."UsersInSegments"."userId" IN ($1) OFFSET $2`;

        // r = await prisma.$queryRaw`CREATE CACHE FROM SELECT "public"."User"."id", "public"."User"."name", "public"."User"."email", "public"."User"."createdAt", "public"."User"."updatedAt" FROM "public"."User" WHERE ("public"."User"."id") IN (SELECT "t1"."userId" FROM "public"."UsersInSegments" AS "t1" LEFT JOIN "public"."Segment" AS "j2" ON ("j2"."id") = ("t1"."segmentId") WHERE ("j2"."tag" = $1 AND ("j2"."id" IS NOT NULL) AND "t1"."userId" IS NOT NULL)) OFFSET $2 `;
        reply.status(200).send({ data: true });
    } catch (error) {
        reply.log.error(error);
        reply.status(500).send();
    }

};
*/
