const formatDate = (inputDate: Date) => {
	const now = new Date();
	const input = new Date(inputDate);

	// Function to add leading zero to single-digit numbers
	const addLeadingZero = (num: number) => (num < 10 ? `0${num}` : num);

	// Check if the input date is today
	if (input.toDateString() === now.toDateString()) {
		return `${addLeadingZero(input.getHours())}:${addLeadingZero(input.getMinutes())}`;
	}

	// Calculate the date for yesterday
	const yesterday = new Date(now);
	yesterday.setDate(now.getDate() - 1);

	// Check if the input date is yesterday
	if (input.toDateString() === yesterday.toDateString()) {
		return 'Yesterday';
	}

	// Format the input date as DD/MM/YYYY
	const day = addLeadingZero(input.getDate());
	const month = addLeadingZero(input.getMonth() + 1); // Months are 0-based
	const year = input.getFullYear();
	return `${day}/${month}/${year}`;
};

export { formatDate };
