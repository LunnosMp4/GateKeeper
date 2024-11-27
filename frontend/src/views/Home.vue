<template>
  <div>
    <h1>Welcome, {{ user.name }}!</h1>
    <p>Email: {{ user.email }}</p>
    <p>Your API token is: {{ user.api_key }}</p>
    <button @click="handleRefreshApiKey">Refresh API Key</button>
    <br />
    <br />
    <router-link v-if="user.permission == 1" to="/admin">Admin Page</router-link>
    <br />
    <br />
    <button @click="handleLogout">Logout</button>
    <br />
    <br />

    <h2>API Key Usage Statistics</h2>
    <button @click="fetchApiKeyUsage(50)">Show Last 50 Requests</button>
    <button @click="fetchApiKeyUsage(100)">Show Last 100 Requests</button>
    <p v-if="loadingStats">Loading stats...</p>
    <p v-if="error">{{ error }}</p>

    <div v-if="stats.length">
      <!-- Key Statistics -->
      <p>Total Requests: {{ stats.length }}</p>
      <p>Success Rate: {{ successRate }}%</p>
      <p>Average Requests per Endpoint: {{ averageRequestsPerEndpoint }}</p>
      <br />

      <!-- Table -->
      <h3>Request Details</h3>
      <table>
        <thead>
          <tr>
            <th>Request Path</th>
            <th>Method</th>
            <th>Status</th>
            <th>Request Time</th>
            <th>IP Address</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="stat in stats" :key="stat.id">
            <td>{{ stat.request_path }}</td>
            <td>{{ stat.request_method }}</td>
            <td :style="{ color: stat.status_code === 200 ? 'green' : 'red' }">
              {{ stat.status_code }}
            </td>
            <td>{{ formatDate(stat.request_time) }}</td>
            <td>{{ stat.request_ip }}</td>
          </tr>
        </tbody>
      </table>

      <br />

      <!-- Charts -->
      <h3>Graphical Analysis</h3>
      <canvas id="endpointChart"></canvas>
      <br />
      <canvas id="statusCodeChart"></canvas>
    </div>
  </div>
</template>

<script>
import Chart from "chart.js/auto";

export default {
  data() {
    return {
      stats: [],
      loadingStats: false,
      error: null,
      endpointChart: null,
      statusCodeChart: null,
    };
  },
  computed: {
    user() {
      return this.$route.meta.user;
    },
    successRate() {
      const successCount = this.stats.filter((stat) => stat.status_code === 200)
        .length;
      return ((successCount / this.stats.length) * 100).toFixed(2);
    },
    averageRequestsPerEndpoint() {
      const endpointCounts = this.stats.reduce((acc, stat) => {
        acc[stat.request_path] = (acc[stat.request_path] || 0) + 1;
        return acc;
      }, {});
      const totalEndpoints = Object.keys(endpointCounts).length;
      return (this.stats.length / totalEndpoints).toFixed(2);
    },
  },
  methods: {
    handleLogout() {
      localStorage.removeItem("authToken");
      this.$router.push("/login");
    },
    handleRefreshApiKey() {
      const authToken = localStorage.getItem("authToken");
      fetch("http://localhost:8080/dashboard/users/refresh_api_key", {
        method: "POST",
        headers: {
          Authorization: `${authToken}`,
        },
      })
        .then((response) => {
          if (!response.ok) {
            throw new Error("Failed to refresh API key");
          }
          return response.json();
        })
        .then((data) => {
          this.$set(this.$route.meta.user, "api_key", data.api_key);
        })
        .catch((error) => {
          alert(error.message);
        });
    },
    fetchApiKeyUsage(size) {
      this.loadingStats = true;
      this.error = null;
      const authToken = localStorage.getItem("authToken");
      fetch(`http://127.0.0.1:8080/dashboard/get_api_key_usage/${size}`, {
        headers: {
          Authorization: `${authToken}`,
        },
      })
        .then((response) => {
          if (!response.ok) {
            throw new Error("Failed to fetch API key usage stats");
          }
          return response.json();
        })
        .then((data) => {
          this.stats = data;
          this.renderCharts();
        })
        .catch((error) => {
          this.error = error.message;
        })
        .finally(() => {
          this.loadingStats = false;
        });
    },
    renderCharts() {
      // Use Vue's nextTick to ensure DOM elements are ready
      this.$nextTick(() => {
        const endpointCounts = this.stats.reduce((acc, stat) => {
          acc[stat.request_path] = (acc[stat.request_path] || 0) + 1;
          return acc;
        }, {});

        const statusCodeCounts = this.stats.reduce((acc, stat) => {
          acc[stat.status_code] = (acc[stat.status_code] || 0) + 1;
          return acc;
        }, {});

        // Destroy existing charts if they exist
        if (this.endpointChart) this.endpointChart.destroy();
        if (this.statusCodeChart) this.statusCodeChart.destroy();

        // Check if canvas elements exist before creating charts
        const endpointCanvas = document.getElementById("endpointChart");
        const statusCodeCanvas = document.getElementById("statusCodeChart");

        if (endpointCanvas) {
          const ctx1 = endpointCanvas.getContext("2d");
          this.endpointChart = new Chart(ctx1, {
            type: "bar",
            data: {
              labels: Object.keys(endpointCounts),
              datasets: [
                {
                  label: "Requests per Endpoint",
                  data: Object.values(endpointCounts),
                },
              ],
            },
          });
        }

        if (statusCodeCanvas) {
          const ctx2 = statusCodeCanvas.getContext("2d");
          this.statusCodeChart = new Chart(ctx2, {
            type: "pie",
            data: {
              labels: Object.keys(statusCodeCounts),
              datasets: [
                {
                  label: "Status Code Distribution",
                  data: Object.values(statusCodeCounts),
                },
              ],
            },
          });
        }
      });
    },
    formatDate(dateString) {
      const date = new Date(dateString);
      return date.toLocaleString();
    },
  },
};
</script>

<style>
table {
  border-collapse: collapse;
  width: 100%;
  margin-top: 1rem;
}
table th,
table td {
  border: 1px solid #ddd;
  padding: 8px;
  text-align: left;
}
table th {
  background-color: #f2f2f2;
}
canvas {
  max-width: 600px;
  margin: 20px auto;
  display: block;
}
</style>
