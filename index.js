const express = require("express");
const bodyparser = require("body-parser");
const cors = require("cors");
require("dotenv").config();

const port = process.env.port || 2004;

const app = express();
app.use(bodyparser.json());
app.use(cors());

app.get("/", (req, res) => {
	res.status(200).send("200: Reached the manczakapi.");
	console.log(`---- served a ${res.statusCode} for: ${req.path}`);
});

app.get("/ip", (req, res) => {
	let ip = req.socket.remoteAddress;
	res.status(200).send(ip);
	console.log(`---- served a ${res.statusCode} for: ${req.path}`);
});

app.get("*", (req, res) => {
	res.status(404).send("404: No data for this path.");
	console.log(`---- served a ${res.statusCode} for: ${req.path}`);
});

app.listen(port, () => {
	console.log(`listening on :${port}`);
});
