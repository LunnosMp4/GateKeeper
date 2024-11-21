<template>
  <div id="app">
    <router-view></router-view>
  </div>
</template>

<style>
/* Example styles */
body {
  font-family: Arial, sans-serif;
  margin: 0;
  padding: 0;
}

form {
  display: flex;
  flex-direction: column;
  width: 300px;
  margin: 0 auto;
}

label {
  margin-top: 10px;
}

input {
  margin-top: 5px;
  padding: 8px;
  font-size: 14px;
}

button {
  margin-top: 20px;
  padding: 10px;
  font-size: 16px;
  cursor: pointer;
}

</style>
