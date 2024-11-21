import { createRouter, createWebHistory } from 'vue-router';
import Login from '../components/Login.vue';
import Register from '../components/Register.vue';
import Home from '../views/Home.vue';

const routes = [
  { path: '/login', component: Login },
  { path: '/register', component: Register },
  {
    path: '/',
    component: Home,
    meta: { requiresAuth: true },
    props: true,
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes
});

router.beforeEach(async (to, from, next) => {
  const authToken = localStorage.getItem("authToken");

  if (to.meta.requiresAuth) {
    if (!authToken) {
      return next("/login");
    }

    try {
      const response = await fetch("http://localhost:8080/dashboard/verify", {
        method: "GET",
        headers: {
          Authorization: `${authToken}`,
        },
      });

      if (response.status === 200) {
        const userData = await response.json();
        // Attach user data to the route's meta property
        to.meta.user = userData;
        next(); // Proceed to the route
      } else {
        localStorage.removeItem("authToken");
        next("/login");
      }
    } catch (error) {
      localStorage.removeItem("authToken");
      next("/login");
    }
  } else {
    next();
  }
});



export default router;
