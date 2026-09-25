const feedback = document.querySelector('#feedback');
const activeSim = document.querySelector('#active-sim');
const volteState = document.querySelector('#volte-state');
const lastAction = document.querySelector('#last-action');
const lastActionTime = document.querySelector('#last-action-time');
const targetOs = document.querySelector('#target-os');
const iosSettings = document.querySelector('#ios-settings');
const defaultApn = document.querySelector('.non-ios-field input');
const connectionSpeed = document.querySelector('#connection-speed');
const connectionDetail = document.querySelector('#connection-detail');
const networkType = document.querySelector('#network-type');
const downloadSpeed = document.querySelector('#download-speed');
const connectionRtt = document.querySelector('#connection-rtt');
const mapStatus = document.querySelector('#map-status');
const mapNodes = document.querySelectorAll('[data-map-node]');

function detectTargetOs() {
  const userAgent = navigator.userAgent.toLowerCase();
  if (/windows phone|windows mobile/.test(userAgent)) return 'Windows Mobile';
  if (/android/.test(userAgent)) return 'Android';
  if (/iphone|ipad|ipod/.test(userAgent)) return 'iOS';
  return 'Android';
}

const detectedOs = detectTargetOs();
targetOs.value = detectedOs;
if (detectedOs === 'iOS') {
  iosSettings.hidden = false;
  defaultApn.required = false;
  iosSettings.querySelectorAll('input').forEach((input) => { input.required = input.name === 'ios_apn'; });
}

async function measureConnection() {
  const connection = navigator.connection || navigator.mozConnection || navigator.webkitConnection;
  const startedAt = performance.now();
  try {
    await fetch(`/styles.css?probe=${Date.now()}`, { cache: 'no-store' });
    const apiRtt = Math.max(0.1, performance.now() - startedAt);
    const rtt = connection?.rtt || apiRtt;
    const downlink = connection?.downlink;
    networkType.textContent = connection?.effectiveType?.toUpperCase() || 'API probe';
    downloadSpeed.textContent = downlink ? `${downlink.toFixed(1)} Mbps` : 'Unavailable';
    connectionRtt.textContent = `${Math.round(rtt)} ms`;
    connectionSpeed.textContent = `${Math.round(rtt)} ms`;
    connectionDetail.textContent = rtt < 1 ? 'Sub-millisecond response' : 'Internet round trip';
  } catch (error) {
    connectionSpeed.textContent = 'Offline';
    connectionDetail.textContent = 'Connection probe failed';
    networkType.textContent = 'Unavailable';
    downloadSpeed.textContent = 'Unavailable';
    connectionRtt.textContent = 'Unavailable';
  }
}

measureConnection();

function setFeedback(message, success = true) {
  feedback.textContent = message;
  feedback.style.color = success ? 'var(--green)' : 'var(--orange)';
  lastActionTime.textContent = new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
}

async function request(endpoint, options = {}) {
  const startedAt = performance.now();
  const isAtomicPath = endpoint.includes('/sim/') || endpoint.includes('/volte/');
  mapNodes.forEach((node) => node.classList.remove('active', 'atomic-active'));
  document.querySelector('[data-map-node="browser"]').classList.add('active');
  mapStatus.textContent = isAtomicPath ? 'Request in flight. Tracking the atomic state write...' : 'Request in flight. Tracking the API and external operation...';
  const response = await fetch(endpoint, { headers: { 'Content-Type': 'application/json' }, ...options });
  if (!response.ok) throw new Error(`Request failed (${response.status})`);
  const elapsed = Math.round(performance.now() - startedAt);
  document.querySelector('[data-map-node="api"]').classList.add('active');
  document.querySelector(`[data-map-node="${isAtomicPath ? 'state' : 'carrier'}"]`).classList.add(isAtomicPath ? 'atomic-active' : 'active');
  mapStatus.textContent = isAtomicPath
    ? `Completed in ${elapsed} ms at the client. The state write is atomic and nanosecond-scale; network time is measured separately.`
    : `Completed in ${elapsed} ms at the client. This path includes API handling and the external operation.`;
  return response.json();
}

document.querySelector('[data-endpoint="sim"]').addEventListener('submit', async (event) => {
  event.preventDefault();
  const slot = event.target.slot.value;
  try {
    const result = await request(`/sim/switch/${slot}`, { method: 'POST' });
    activeSim.textContent = `Slot ${slot}`;
    lastAction.textContent = 'SIM switched';
    setFeedback(result);
  } catch (error) { setFeedback(error.message, false); }
});

document.querySelector('[data-endpoint="volte"]').addEventListener('submit', async (event) => {
  event.preventDefault();
  const inCall = event.target.in_call.checked;
  try {
    const result = await request('/volte/data-switch', { method: 'POST', body: JSON.stringify(inCall) });
    volteState.textContent = inCall ? 'In call' : 'Standby';
    lastAction.textContent = 'VoLTE updated';
    setFeedback(result);
  } catch (error) { setFeedback(error.message, false); }
});

document.querySelector('[data-endpoint="apn"]').addEventListener('submit', async (event) => {
  event.preventDefault();
  const data = Object.fromEntries(new FormData(event.target));
  if (data.os_target === 'iOS') {
    data.apn = data.ios_apn;
    data.username = data.ios_username;
    data.password = data.ios_password;
  }
  delete data.ios_apn;
  delete data.ios_username;
  delete data.ios_password;
  try {
    const result = await request('/apn/config', { method: 'POST', body: JSON.stringify(data) });
    lastAction.textContent = 'APN registered';
    setFeedback(result);
  } catch (error) { setFeedback(error.message, false); }
});

document.querySelector('[data-endpoint="message"]').addEventListener('submit', async (event) => {
  event.preventDefault();
  const data = Object.fromEntries(new FormData(event.target));
  data.is_rcs = event.target.is_rcs.checked;
  try {
    const result = await request('/messaging/send', { method: 'POST', body: JSON.stringify(data) });
    lastAction.textContent = 'Message sent';
    setFeedback(result);
  } catch (error) { setFeedback(error.message, false); }
});
