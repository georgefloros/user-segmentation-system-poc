## ReadySet queries to be cached
- `CREATE CACHE FROM SELECT "public"."Segment"."id", "public"."Segment"."title", "public"."Segment"."description", "public"."Segment"."tag", "public"."Segment"."whereStatement", "public"."Segment"."isGeneric", "public"."Segment"."createdAt", "public"."Segment"."updatedAt" FROM "public"."Segment" WHERE 1=1 OFFSET $1`
- `CREATE CACHE FROM SELECT "public"."Segment"."id", "public"."Segment"."title", "public"."Segment"."description", "public"."Segment"."tag", "public"."Segment"."whereStatement", "public"."Segment"."isGeneric", "public"."Segment"."createdAt", "public"."Segment"."updatedAt" FROM "public"."Segment" WHERE ("public"."Segment"."id" = $1 AND 1=1) LIMIT $2 OFFSET $3`
- `CREATE CACHE FROM SELECT "public"."Activity"."id", "public"."Activity"."title", "public"."Activity"."description", "public"."Activity"."code", "public"."Activity"."createdAt", "public"."Activity"."updatedAt" FROM "public"."Activity" WHERE 1=1 OFFSET $1`
- `CREATE CACHE FROM SELECT "public"."Activity"."id", "public"."Activity"."title", "public"."Activity"."description", "public"."Activity"."code", "public"."Activity"."createdAt", "public"."Activity"."updatedAt" FROM "public"."Activity" WHERE ("public"."Activity"."id" = $1 AND 1=1) LIMIT $2 OFFSET $3`
- `CREATE CACHE FROM SELECT "public"."User"."id", "public"."User"."name", "public"."User"."email", "public"."User"."createdAt", "public"."User"."updatedAt" FROM "public"."User" WHERE ("public"."User"."id" = $1 AND 1=1) LIMIT $2 OFFSET $3`
- `CREATE CACHE FROM SELECT "public"."UsersInSegments"."userId", "public"."UsersInSegments"."segmentId", "public"."UsersInSegments"."createdAt", "public"."UsersInSegments"."updatedAt" FROM "public"."UsersInSegments" WHERE "public"."UsersInSegments"."userId" IN ($1) OFFSET $2`
- `CREATE CACHE FROM SELECT "public"."User"."id", "public"."User"."name", "public"."User"."email", "public"."User"."createdAt", "public"."User"."updatedAt" FROM "public"."User" WHERE ("public"."User"."id") IN (SELECT "t1"."userId" FROM "public"."UsersInSegments" AS "t1" LEFT JOIN "public"."Segment" AS "j2" ON ("j2"."id") = ("t1"."segmentId") WHERE ("j2"."tag" = $1 AND ("j2"."id" IS NOT NULL) AND "t1"."userId" IS NOT NULL)) OFFSET $2`

# Event sample 
```json
{     
    
	"id":  "123232", 
	"id_type":  "cookie",
 	"region": "California",
 	"device_type": "mobile",
 	"country": "US",
    "created_at":"2023-10-31T00:00:00",
 	"payload": {  
     		"activity_type": "page_viewed",
     		"url": "http://myawesomeproduct",
     		"order_total":"",
     		"order_id": "",
            "element_id": ""
    }
} 
``` 
