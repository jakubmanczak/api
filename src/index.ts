import express from "express";
import cors from "cors";
require("dotenv").config();

import splashjson from "./data/splash.json";

const port = process.env.PORT || 2004;
const app = express();
app.use(cors());

app.get("/", (req, res) => {
	res.status(200).send("200: Reached the manczakapi.");
	console.log(`---- served a ${res.statusCode} for: ${req.path}`);
});
app.get("/splash", (req, res) => {
	res
		.status(200)
		.send(
			splashjson.generic[Math.floor(Math.random() * splashjson.generic.length)]
		);
	console.log(`---- served a ${res.statusCode} for: ${req.path}`);
});
app.get("*", (req, res) => {
	res.status(404).send("404: Reached the manczakapi.");
	console.log(`---- served a ${res.statusCode} for: ${req.path}`);
});

app.listen(port, () => {
	console.log(`listening on :${port}`);
});
