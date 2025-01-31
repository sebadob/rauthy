export interface PropsInputDate {
    id?: string,
    name?: string,
    value?: string,
    label?: string,
    title?: string,
    disabled?: boolean,
    min?: string,
    max?: string,
    required?: boolean,

    onEnter?: () => void,
    onLeft?: () => void,
    onRight?: () => void,
    onUp?: () => void,
    onDown?: () => void,
}

export interface PropsInputDateCombo extends PropsInputDate {
    withDelete?: boolean,
}