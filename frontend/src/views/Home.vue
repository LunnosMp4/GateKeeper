<template>
  <div>
    <h1>Welcome, {{ user.name }}!</h1>
    <p>Email: {{ user.email }}</p>
    <button @click="handleLogout">Logout</button>
  </div>
</template>

<script>
export default {
  computed: {
    user() {
      return this.$route.meta.user;
    },
  },
  methods: {
    handleLogout() {
      localStorage.removeItem('authToken');
      this.$router.push('/login');
    }
  }
};
</script>