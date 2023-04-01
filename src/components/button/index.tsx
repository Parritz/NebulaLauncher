import React from "react";
import { Inter } from "next/font/google";
import styles from "./button.module.css";
import { TButton } from "./types";

const inter = Inter({ subsets: ["latin"] });
export class Button extends React.Component<TButton> {
	constructor(props: TButton) {
		super(props);
	}

	render() {
		return (
			<div className={styles["button-container"]}>
				<button className={styles.button} onClick={this.props.onClick} style={{["background-color" as never]: this.props.color}}>
					{this.props.text}
				</button>
			</div>
		);
	}
}