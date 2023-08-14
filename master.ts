import { Application, Router } from "https://deno.land/x/oak@v12.6.0/mod.ts";
import { oakCors } from "https://deno.land/x/cors@v1.2.2/mod.ts";
import { DB } from "https://deno.land/x/sqlite@v3.7.3/mod.ts";

const app = new Application();
const r = new Router();
const dbpath = "../manczak.db";

const db = new DB(dbpath);
db.execute(`
  CREATE TABLE IF NOT EXISTS splashes (
    splashid BLOB NOT NULL DEFAULT 'replace with ulid' UNIQUE,
    splash TEXT NOT NULL,
    PRIMARY KEY("splashid")
  );
`);
// in this case, "secured" is an integer wherein
// 0: unprotected,      1: read protected
// 2: write protected,  3: read&write protected
db.execute(`
  CREATE TABLE IF NOT EXISTS vars (
    id TEXT NOT NULL DEFAULT 'replace with ulid' UNIQUE,
    secured INTEGER NOT NULL DEFAULT 0,
    varname TEXT NOT NULL DEFAULT 'replace with var name' UNIQUE,
    varbody BLOB,
    PRIMARY KEY("id")
  );
`);
db.close();

app.addEventListener("listen", () => {
  console.log("Running.");
});

r.get("/", (ctx) => {
  let res = "200 (OK). Available endpoints:";
  r.forEach((el) => {
    res = res.concat("\n", el.methods.toString(), ": ", el.path);
  });
  ctx.response.body = res;
});

r.get("/splash", (ctx) => {
  const db = new DB(dbpath);
  for (const [splash] of db.query(`
    SELECT splash FROM splashes
    ORDER BY RANDOM() LIMIT 1
  `)) {
    ctx.response.body = splash as string;
  }
  db.close();
});

r.get("/splash/:id", (ctx) => {
  const db = new DB(dbpath);
  const query = db.prepareQuery(
    "SELECT splash FROM splashes WHERE splashid = :id"
  );
  const [qres] = query.one({ id: ctx.params.id });
  query.finalize();
  db.close();
  if (qres) {
    ctx.response.body = qres;
  } else {
    ctx.response.status = 400;
    ctx.response.body = `No splash with id ${ctx.params.id}`;
  }
});

r.get("/splashes", (ctx) => {
  const db = new DB(dbpath);
  const res: { id: string; splash: string }[] = [];
  for (const [id, splash] of db.query(
    `SELECT * FROM splashes ORDER BY splashid`
  )) {
    res.push({ id: id as string, splash: splash as string });
  }
  ctx.response.body = res;
  db.close();
});

app.use(oakCors());
app.use(r.routes());
app.use(r.allowedMethods());
await app.listen({ port: 2004 });
// TODO: figure out how to host this on default ports
// automatically and find an unoccupied one automatically,
// as well as report it
