import React from "react";
import { Inter } from "next/font/google";
import styles from "./modal.module.css";
import globalStyles from "@styles/globalLib.module.css";

import { TModal } from "./types"

import { Button } from "@components/button";

const inter = Inter({ subsets: ["latin"] });
export class Modal extends React.Component<TModal> {
	private testingStyles: TModal;

	constructor(props: TModal) {
		super(props);

		this.testingStyles = {
			width: props.width,
			height: props.height
		}
	}
	
	render() {
		return (
			<div className={styles["modal-window"]} style={this.testingStyles}>
				<h2 className={styles.title}>{this.props.title}</h2>
				<div className={globalStyles["flex-container"]}>
					<input placeholder="Github link" className={styles.input}></input>
				</div>
				<Button text="Confirm" color={"var(--mg2-color)"}></Button>
			</div>
		);
	}
}
