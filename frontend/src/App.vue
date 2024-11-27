<template>
  <div id="app">
    <router-view></router-view>
  </div>
</template>

<style>
@import url('https://fonts.googleapis.com/css2?family=Roboto:wght@400;500&display=swap');

html, body, #app {
  margin: 0;
  padding: 0;
  background-color: #a6c1ee;
  display: flex;
  justify-content: center;
  align-items: flex-start;
  font-family: 'Roboto', sans-serif;
}

button {
  padding: 10px 20px;
  background: #007BFF;
  color: white;
  font-size: 16px;
  margin-right: 10px;
  border: none;
  border-radius: 5px;
  cursor: pointer;
  transition: background-color 0.3s;
}

button:hover {
  background: #0056b3;
}

</style>
