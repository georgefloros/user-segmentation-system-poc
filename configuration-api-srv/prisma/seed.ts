import { PrismaClient, Prisma } from '@prisma/client';
import { faker } from '@faker-js/faker';

const prisma = new PrismaClient();


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

}
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




async function main() {
  console.log(`Start seeding ...`);
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
        clientRefId: faker.string.uuid()
      },
    });
    console.log(`Created user with id: ${user.id}`);
  }
  const activityData = getActivityData();
  for (const a of activityData) {
    const activity = await prisma.activity.create({
      data: a,
    });
    console.log(`Created activity with id: ${activity.id}`);
  }

  const segmentData = getSegmentData();
  for (const s of segmentData) {
    const segment = await prisma.segment.create({
      data: s,
    });
    console.log(`Created segment with id: ${segment.id}`);
  }

  //create relations
  const activity = await prisma.activity.findFirst({ where: { code: 'page_viewed' } });
  const segment = await prisma.segment.findFirst({ where: { tag: 'mobile_no_recent_activity' } });
  if (activity && segment) {
    const segmentActivitiesData = getSegmentActivityData([{ activityId: activity.id, segmentId: segment.id }]);
    for (const sa of segmentActivitiesData) {
      const segmentActivity = await prisma.segmentActivities.create({
        data: sa,
      });
      console.log(`Created segment:${segmentActivity.segmentId} - activity:${segmentActivity.segmentId} relation`);
    }
  }

  console.log(`Seeding finished.`);
}


main()
  .then(async () => {
    await prisma.$disconnect();
  })
  .catch(async (e) => {
    console.error(e);
    await prisma.$disconnect();
    process.exit(1);
  });
