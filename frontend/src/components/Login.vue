<template>
  <div class="login">
    <h2>Login to your account</h2>
    <form @submit.prevent="handleLogin" class="login-form">
      <div class="form-group">
        <label for="email">Email</label>
        <input type="email" id="email" v-model="email" required />
      </div>
      <div class="form-group">
        <label for="password">Password</label>
        <input type="password" id="password" v-model="password" required />
      </div>
      <button type="submit">Login</button>
    </form>
    <p class="register-link">
      Don't have an account? <router-link to="/register">Register here</router-link>.
    </p>
  </div>
</template>

<script>
export default {
  data() {
    return {
      email: '',
      password: ''
    };
  },
  methods: {
    async handleLogin() {
      try {
        const response = await fetch('http://localhost:8080/login', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ email: this.email, password: this.password })
        });

        if (!response.ok) {
          throw new Error('Login failed');
        }

        const data = await response.json();
        localStorage.setItem('authToken', data);
        this.$router.push('/');
      } catch (error) {
        alert(error.message);
      }
    }
  }
};
</script>

<style>
.login {
  width: 400px;
  margin: 0 auto;
  margin-top: 100px;
  padding: 20px;
  background: #fff;
  border-radius: 10px;
  box-shadow: 0 4px 10px rgba(0, 0, 0, 0.1);
  text-align: center;
}

.login h2 {
  font-size: 24px;
  color: #333;
  margin-bottom: 20px;
}

.login-form {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.form-group {
  display: flex;
  flex-direction: column;
  text-align: left;
}

label {
  font-size: 14px;
  margin-bottom: 5px;
  color: #555;
}

input {
  padding: 10px;
  font-size: 14px;
  border: 1px solid #ccc;
  border-radius: 5px;
  transition: border-color 0.3s;
}

input:focus {
  border-color: #007BFF;
  outline: none;
  box-shadow: 0 0 5px rgba(0, 123, 255, 0.3);
}

.register-link {
  margin-top: 20px;
  font-size: 14px;
  color: #333;
}

.register-link a {
  color: #007BFF;
  text-decoration: none;
  transition: color 0.3s;
}

.register-link a:hover {
  color: #0056b3;
}
</style>
