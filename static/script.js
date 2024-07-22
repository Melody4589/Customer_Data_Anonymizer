document.getElementById('data-form').addEventListener('submit', async function(event) {
    event.preventDefault();

    const name = document.getElementById('name').value;
    const email = document.getElementById('email').value;
    const phone = document.getElementById('phone').value;

    const response = await fetch('/anonymize', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ name, email, phone })
    });

    if (response.ok) {
        const result = await response.json();
        document.getElementById('result').innerHTML = `
            <p>Anonymized ID: ${result.id}</p>
            <p>Anonymized Name: ${result.anonymized_name}</p>
            <p>Anonymized Email: ${result.anonymized_email}</p>
            <p>Anonymized Phone: ${result.anonymized_phone}</p>
        `;
    } else {
        document.getElementById('result').innerHTML = `<p>Error occurred</p>`;
    }
});
