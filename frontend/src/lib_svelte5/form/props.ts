export interface PropsInputDate {
    id?: string,
    name?: string,
    value?: string,
    label?: string,
    errMsg?: string,
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
    timeName?: string,
    timeValue?: string,
    timeErrMsg?: string,
    timeMin?: string,
    timeMax?: string,

    timeDisabled?: boolean,
    timeRequired?: boolean,

    withDelete?: boolean,
    withTime?: boolean,
}
