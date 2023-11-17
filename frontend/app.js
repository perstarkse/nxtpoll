
document.addEventListener('DOMContentLoaded', function () {
    const darkModeToggle = document.getElementById('darkModeToggle');

    darkModeToggle.addEventListener('click', function () {
        document.body.classList.toggle('bg-gray-800');
        document.body.classList.toggle('text-white');
        this.classList.toggle('bg-gray-600');
    });

    // Placeholder for HTMX dynamic content interactions
    // Example: HTMX AJAX calls for loading and submitting polls
});
