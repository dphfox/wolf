export default function debounce<T extends (...args: any[]) => any>(func: T, delay_ms: number): (...args: Parameters<T>) => void {
	let timeout_id: ReturnType<typeof setTimeout> | null = null;
	let is_waiting = false;
	return function(...args: Parameters<T>): void {
		// If not waiting, execute immediately
		if (!is_waiting) {
			func(...args);
			is_waiting = true;

			// Set timeout to reset the waiting flag
			timeout_id = setTimeout(() => {
				is_waiting = false;
			}, delay_ms);
		}
	};
}