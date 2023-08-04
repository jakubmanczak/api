import { Application, Router } from "https://deno.land/x/oak@v12.6.0/mod.ts";
import { oakCors } from "https://deno.land/x/cors@v1.2.2/mod.ts";
import { DB } from "https://deno.land/x/sqlite@v3.7.3/mod.ts";

const app = new Application();
const r = new Router();
const dbpath = "../manczak.db";

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

r.get("/splashes", (ctx) => {
  const db = new DB(dbpath);
  const res: string[] = [];
  for (const [splash] of db.query(
    `SELECT splash FROM splashes ORDER BY splashid`
  )) {
    res.push(splash as string);
  }
  ctx.response.body = res;
  db.close();
});

app.use(oakCors());
app.use(r.routes());
app.use(r.allowedMethods());
await app.listen({ port: 2004 });