import React, { useState } from "react";
import { Inter } from "next/font/google";
import styles from "./navbar.module.css";
import globalStyles from "@styles/globalLib.module.css";

import { Button } from "@components/button";
import { Modal } from "@components/modal";

const inter = Inter({ subsets: ["latin"] });
export default function Navbar() {
	const [active, setActive] = useState("CurseForge");
	return (<div></div>)
};
