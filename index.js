const express = require("express");
const bodyparser = require("body-parser");
const cors = require("cors");

const app = express();
app.use(bodyparser.json());
app.use(cors());

app.get("/", (req, res) => {
	res.status(204).send("You've reached the manczakapi.");
	console.log("-- served a / response");
});
app.get("/remoteip", (req, res) => {
	res.json(req.socket.remoteAddress);
	console.log(`-- served a /remoteip response for ${ip}`);
});
app.get("/splash", (req, res) => {
	res.json(require("./splash.json"));
	console.log("-- served a /splash response");
});
app.get("/splash/generic", (req, res) => {
	res.json(require("./splash.json").generic);
	console.log("-- served a /splash/generic response");
});
app.get("/splash/videogame", (req, res) => {
	res.json(require("./splash.json").videogame);
	console.log("-- served a /splash/videogame response");
});
app.get("*", (req, res) => {
	res.status(400).send("There isn't any response provided for such path.");
});

app.listen(2004, () => {
	console.log(`listening on :2004`);
});
