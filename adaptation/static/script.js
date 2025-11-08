// API base URL
const API_BASE = '';

// DOM elements
const runButton = document.getElementById('runSimulation');
const resetButton = document.getElementById('resetSimulation');
const statusDiv = document.getElementById('status');
const resultsDiv = document.getElementById('results');
const iterationsInput = document.getElementById('iterations');
const moveLimitInput = document.getElementById('moveLimit');

// Update status message
function updateStatus(message, type = 'ready') {
    statusDiv.textContent = message;
    statusDiv.className = `status-message ${type}`;
}

// Display results
function displayResults(data) {
    resultsDiv.innerHTML = '';
    
    const resultItem = document.createElement('div');
    resultItem.className = 'result-item';
    
    resultItem.innerHTML = `
        <h3>Simulation Completed</h3>
        <p><strong>Status:</strong> <span class="result-value">${data.status || 'Success'}</span></p>
        <p><strong>Message:</strong> ${data.message || 'Simulation completed successfully'}</p>
        <p><strong>Timestamp:</strong> <span class="result-value">${new Date().toLocaleString()}</span></p>
    `;
    
    resultsDiv.appendChild(resultItem);
}

// Display error
function displayError(error) {
    resultsDiv.innerHTML = `
        <div class="result-item" style="border-left-color: #dc3545;">
            <h3 style="color: #dc3545;">Error</h3>
            <p>${error}</p>
        </div>
    `;
}

// Run simulation
async function runSimulation() {
    const iterations = parseInt(iterationsInput.value);
    const moveLimit = parseInt(moveLimitInput.value);
    
    if (isNaN(iterations) || iterations < 1) {
        updateStatus('Please enter a valid number of iterations', 'error');
        return;
    }
    
    if (isNaN(moveLimit) || moveLimit < 1) {
        updateStatus('Please enter a valid move limit', 'error');
        return;
    }
    
    // Disable buttons during simulation
    runButton.disabled = true;
    resetButton.disabled = true;
    updateStatus('Running simulation...', 'running');
    
    try {
        const response = await fetch('/api/run', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                iterations: iterations,
                move_limit: moveLimit
            })
        });
        
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        
        const data = await response.json();
        displayResults(data);
        updateStatus('Simulation completed successfully', 'success');
    } catch (error) {
        console.error('Error running simulation:', error);
        displayError(error.message);
        updateStatus('Error running simulation', 'error');
    } finally {
        // Re-enable buttons
        runButton.disabled = false;
        resetButton.disabled = false;
    }
}

// Reset simulation
function resetSimulation() {
    iterationsInput.value = '100';
    moveLimitInput.value = '10';
    resultsDiv.innerHTML = '<p class="placeholder">Run a simulation to see results here...</p>';
    updateStatus('Ready to start simulation', 'ready');
}

// Event listeners
runButton.addEventListener('click', runSimulation);
resetButton.addEventListener('click', resetSimulation);

// Allow Enter key to trigger simulation
iterationsInput.addEventListener('keypress', (e) => {
    if (e.key === 'Enter') {
        runSimulation();
    }
});

moveLimitInput.addEventListener('keypress', (e) => {
    if (e.key === 'Enter') {
        runSimulation();
    }
});

// Initialize status
updateStatus('Ready to start simulation', 'ready');
