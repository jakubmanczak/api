import express from "express";
import cors from "cors";
import path from "path";
require("dotenv").config();

import splshjson from "./data/splash.json";

const port = process.env.PORT || 2004;
const app = express();
app.use(cors());

app.get("/", (req, res) => {
	res.status(200).send("200: Reached the manczakapi.");
});

app.get("/splash", (req, res) => {
	let splsh: string[] = [];
	if (req.query["generic"] !== undefined) splsh.push(...splshjson.generic);
	if (req.query["personal"] !== undefined) splsh.push(...splshjson.personal);
	if (req.query["games"] !== undefined) splsh.push(...splshjson.games);
	if (splsh.length === 0) {
		splsh.push(...splshjson.generic, ...splshjson.games);
	}
	if (req.query["maxlength"]) {
		splsh = splsh.filter((el: string) => {
			return el.length <= parseInt(req.query["maxlength"] as string);
		});
	}
	if (req.query["all"] !== undefined) {
		return res.status(200).send(splsh);
	}
	res.status(200).send(splsh[Math.floor(Math.random() * splsh.length)]);
});

app.get("/github-profile-readme-img", (req, res) => {
	res.sendFile("./data/github.jpg", {
		root: path.join(__dirname),
	});
});

app.get("*", (req, res) => {
	res.status(404).send("404: Not Found.");
});

app.listen(port, () => {
	console.log(`listening on :${port}`);
});
