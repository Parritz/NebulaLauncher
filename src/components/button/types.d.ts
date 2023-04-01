// TO-DO: Color types should be in a global types file.
type RGB = `rgb(${number}, ${number}, ${number})`;
type RGBA = `rgba(${number}, ${number}, ${number}, ${number})`;
type HEX = `#${string}`;
type VAR = `var(${string})`;
export type TColor = RGB | RGBA | HEX | VAR;

export type TButton = {
	text: string;
	color: TColor;
	onClick?: MouseEventHandler<HTMLButtonElement>;
}