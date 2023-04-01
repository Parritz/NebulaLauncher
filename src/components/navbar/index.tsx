import React, { useState } from "react";
import { Inter } from "next/font/google";
import styles from "./navbar.module.css";

import { Button } from "@components/button";
import { Modal } from "@components/modal";

function switchWindow(type: string) {
	switch(type) {
		case "CurseForge": {
			console.log("CurseForge window pressed.");
			break;
		}
		case "Modrinth": {
			console.log("Modrinth window pressed.");
			break;
		}
		case "Custom": {
			console.log("Custom window pressed.");
			
		}
		default: {
			console.log("Unknown window pressed.");
			break;
		}
	}
}

const inter = Inter({ subsets: ["latin"] });
export default function Navbar() {
	const [active, setActive] = useState("CurseForge");
	return (
		<nav>
		<a href="./"><h2>MC Mod Manager</h2></a>

		<h3>Install</h3>
		<div className={styles["flex-container"]}>
			
			<ul>
				<li onClick={() => setActive("CurseForge")}><a>CurseForge</a></li>
				<li onClick={() => setActive("Modrinth")}><a>Modrinth</a></li>
				<li onClick={() => setActive("Custom")}><a>Custom</a></li>
			</ul>

			{active === "Custom" && <Modal title="test" width="200px" height="150px"></Modal>}
		</div>

		<h3>Create</h3>
		<p>Coming soon!</p>
	</nav>)
};
