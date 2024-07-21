<script setup lang="ts">
import logo from './assets/logo.png'
</script>

<template>
  <v-app>
    <v-card>
      <v-layout>
        <v-navigation-drawer
            image="@/assets/logo.png"
            theme="dark"
            permanent
        >
          <template v-slot:prepend>
            <v-list-item
                lines="two"
                :prepend-avatar="logo"
                subtitle="Yet Another"
                title="Unlock Checker"
            ></v-list-item>
          </template>

          <v-divider></v-divider>

          <v-list nav>
            <v-list-item prepend-icon="mdi-email" title="常规检测" value="inbox"></v-list-item>
            <v-list-item prepend-icon="mdi-account-supervisor-circle" title="解锁检测" value="supervisors"></v-list-item>
            <v-list-item prepend-icon="mdi-clock-start" title="Clock-in" value="clockin"></v-list-item>
          </v-list>
        </v-navigation-drawer>
        <v-main style="height: 100vh"></v-main>
      </v-layout>
    </v-card>
  </v-app>
</template>

<style scoped>
</style>
