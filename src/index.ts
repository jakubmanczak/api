import express from "express";
import cors from "cors";
require("dotenv").config();

import splashjson from "./data/splash.json";

const port = process.env.PORT || 2004;
const app = express();
app.use(cors());

app.get("/", (req, res) => {
	res.status(200).send("200: Reached the manczakapi.");
});

app.get(["/splas:plural", "/splas:plural/:type"], (req, res) => {
	if (req.params.plural != "h" && req.params.plural != "hes") {
		return res.status(404).send("404: Not Found."); // feign "bad endpoint"
	}
	let splashes = [];
	// TODO: optimise the everloving fuck out of this.
	if (!req.params.type) {
		splashes.push(splashjson.generic);
		splashes.push(splashjson.videogame);
		splashes.push(splashjson.music);
	}
	if (req.params.type == "all") {
		splashes.push(splashjson.generic);
		splashes.push(splashjson.personal);
		splashes.push(splashjson.videogame);
		splashes.push(splashjson.music);
	}
	if (req.params.type == "generic") splashes.push(splashjson.generic);
	if (req.params.type == "personal") splashes.push(splashjson.personal);
	if (req.params.type == "videogame") splashes.push(splashjson.videogame);
	if (req.params.type == "music") splashes.push(splashjson.music);
	splashes = splashes.flat();
	// TODO: implement splash maxlength query
	if (splashes.length == 0) {
		return res.status(400).send("400: Bad splash type specifier");
	}
	if (req.params.plural == "h") {
		return res
			.status(200)
			.send(splashes[Math.floor(Math.random() * splashes.length)]);
	}
	return res.status(200).send(splashes);
});

app.get("*", (req, res) => {
	res.status(404).send("404: Not Found.");
});

app.listen(port, () => {
	console.log(`listening on :${port}`);
});
