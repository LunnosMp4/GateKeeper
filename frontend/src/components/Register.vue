<template>
  <div class="register">
    <h2>Create an account</h2>
    <form @submit.prevent="handleRegister" class="register-form">
      <div class="form-group">
        <label for="name">Name</label>
        <input type="text" id="name" v-model="name" required />
      </div>
      <div class="form-group">
        <label for="email">Email</label>
        <input type="email" id="email" v-model="email" required />
      </div>
      <div class="form-group">
        <label for="password">Password</label>
        <input type="password" id="password" v-model="password" required />
      </div>
      <button type="submit">Register</button>
    </form>
    <p class="login-link">
      Already have an account? <router-link to="/login">Login here</router-link>.
    </p>
  </div>
</template>

<script>
export default {
  data() {
    return {
      name: '',
      email: '',
      password: ''
    };
  },
  methods: {
    async handleRegister() {
      try {
        const response = await fetch('http://localhost:8080/register', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ name: this.name, email: this.email, password: this.password })
        });

        if (!response.ok) {
          throw new Error('Registration failed');
        }

        alert('Registration successful! You can now log in.');
        this.$router.push('/login');
      } catch (error) {
        alert(error.message);
      }
    }
  }
};
</script>

<style>
/* Base styles for the register container */
.register {
  width: 400px;
  margin: 0 auto;
  padding: 20px;
  margin-top: 100px;
  background: #fff;
  border-radius: 10px;
  box-shadow: 0 4px 10px rgba(0, 0, 0, 0.1);
  text-align: center;
}

/* Heading styles */
.register h2 {
  font-size: 24px;
  color: #333;
  margin-bottom: 20px;
}

/* Form styles */
.register-form {
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

/* Login link styles */
.login-link {
  margin-top: 20px;
  font-size: 14px;
  color: #333;
}

.login-link a {
  color: #007BFF;
  text-decoration: none;
  transition: color 0.3s;
}

.login-link a:hover {
  color: #0056b3;
}
</style>
