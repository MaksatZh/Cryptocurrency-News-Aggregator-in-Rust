document.getElementById("searchForm").addEventListener("submit", async function (e) {
    e.preventDefault();

    const symbol = document.getElementById("symbolInput").value.trim().toUpperCase();
    const container = document.getElementById("newsContainer");

    container.innerHTML = "<p>🔄 Loading news...</p>";

    try {
        const response = await fetch(`/api/news/${symbol}`);
        if (!response.ok) throw new Error("Failed to fetch news");

        const data = await response.json();

        if (data.length === 0) {
            container.innerHTML = "<p>No news found.</p>";
            return;
        }

        container.innerHTML = data.map(item => `
            <div class="news-item">
                <h3>${item.title}</h3>
                <p><strong>Source:</strong> ${item.source}</p>
                <p><strong>Date:</strong> ${item.published_at}</p>
                <p>${item.summary}</p>
                <a href="${item.url}" target="_blank">Read more →</a>
            </div>
        `).join('');
    } catch (error) {
        container.innerHTML = `<p> Error: ${error.message}</p>`;
    }
});
