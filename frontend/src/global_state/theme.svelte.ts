let isDark: undefined | boolean = $state();

export function useTheme() {
    return {
        isDark(): undefined | boolean {
            return isDark;
        },
        setIsDark(value: boolean) {
            isDark = value;
        },
        toggle() {
            isDark = !isDark;
        },
    };
}
