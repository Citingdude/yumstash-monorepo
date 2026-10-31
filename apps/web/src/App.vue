<script setup lang="ts">
import { computed, onMounted, reactive, ref } from 'vue'
import { api } from './api/client'
import type { components } from './api/schema'

type Recipe = components['schemas']['Recipe']
type RecipeInput = components['schemas']['RecipeInput']

const recipes = ref<Recipe[]>([])
const loading = ref(true)
const saving = ref(false)
const error = ref('')
const search = ref('')
const editingId = ref<number | null>(null)
const showForm = ref(false)
const form = reactive<RecipeInput>({ title: '', description: '', ingredients: '', instructions: '' })
const filtered = computed(() => {
  const query = search.value.trim().toLowerCase()
  return recipes.value.filter(recipe => `${recipe.title} ${recipe.description} ${recipe.ingredients}`.toLowerCase().includes(query))
})

async function loadRecipes() {
  loading.value = true
  error.value = ''
  try {
    const result = await api.GET('/api/recipes')
    if (result.error || !result.data) throw new Error('Could not load recipes. Check that the API and database are running.')
    recipes.value = result.data
  } catch (e) { error.value = e instanceof Error ? e.message : 'Could not load recipes.' }
  finally { loading.value = false }
}
function resetForm() {
  editingId.value = null
  Object.assign(form, { title: '', description: '', ingredients: '', instructions: '' })
  showForm.value = false
}
function edit(recipe: Recipe) {
  editingId.value = recipe.id
  Object.assign(form, { title: recipe.title, description: recipe.description, ingredients: recipe.ingredients, instructions: recipe.instructions })
  error.value = ''
  showForm.value = true
  window.scrollTo({ top: 0, behavior: 'smooth' })
}
async function save() {
  if (!form.title.trim()) { error.value = 'Please enter a recipe title.'; return }
  saving.value = true
  error.value = ''
  try {
    const body = { ...form, title: form.title.trim() }
    const result = editingId.value === null
      ? await api.POST('/api/recipes', { body })
      : await api.PUT('/api/recipes/{id}', { params: { path: { id: editingId.value } }, body })
    if (result.error || !result.data) throw new Error(result.error?.message || 'Could not save recipe.')
    const saved = result.data
    recipes.value = editingId.value === null ? [saved, ...recipes.value] : recipes.value.map(recipe => recipe.id === saved.id ? saved : recipe)
    resetForm()
  } catch (e) { error.value = e instanceof Error ? e.message : 'Could not save recipe.' }
  finally { saving.value = false }
}
async function remove(recipe: Recipe) {
  if (!window.confirm(`Delete “${recipe.title}”?`)) return
  error.value = ''
  try {
    const result = await api.DELETE('/api/recipes/{id}', { params: { path: { id: recipe.id } } })
    if (result.error || !result.response.ok) throw new Error(result.error?.message || 'Could not delete recipe.')
    recipes.value = recipes.value.filter(item => item.id !== recipe.id)
    if (editingId.value === recipe.id) resetForm()
  } catch (e) { error.value = e instanceof Error ? e.message : 'Could not delete recipe.' }
}
onMounted(loadRecipes)
</script>

<template>
  <div class="shell">
    <header class="topbar"><a class="brand" href="#top"><span class="brand-icon">✳</span> yumstash</a><span class="top-label">YOUR RECIPE COLLECTION</span></header>
    <main id="top">
      <section class="hero">
        <div><p class="eyebrow">YOUR KITCHEN, ORGANIZED</p><h1>Good food starts<br><em>right here.</em></h1><p class="intro">A little home for all the recipes you love. Save your favorites and make something delicious.</p><button class="primary" @click="resetForm(); showForm = true">＋ &nbsp; Add a recipe</button></div>
        <div class="hero-art" aria-hidden="true"><span class="plate">✳</span><span class="art-caption">made with love<br>saved for later</span></div>
      </section>
      <div v-if="error" class="notice" role="alert">{{ error }} <button class="link" @click="loadRecipes">Retry loading</button></div>
      <section v-if="showForm" class="form-card" aria-labelledby="form-title">
        <div class="section-heading"><div><p class="eyebrow">YOUR COLLECTION</p><h2 id="form-title">{{ editingId === null ? 'Add a recipe' : 'Edit recipe' }}</h2></div><button class="link" type="button" @click="resetForm">Cancel</button></div>
        <form @submit.prevent="save">
          <label>Recipe name <input v-model="form.title" required maxlength="200" placeholder="e.g. Sunday tomato pasta"></label>
          <label>Description <input v-model="form.description" placeholder="A few words about this recipe"></label>
          <div class="form-grid"><label>Ingredients <textarea v-model="form.ingredients" rows="5" placeholder="Add ingredients, one per line"></textarea></label><label>Instructions <textarea v-model="form.instructions" rows="5" placeholder="How do you make it?"></textarea></label></div>
          <button class="primary" :disabled="saving">{{ saving ? 'Saving…' : 'Save recipe' }}</button>
        </form>
      </section>
      <section class="collection" aria-labelledby="recipes-title">
        <div class="section-heading"><div><p class="eyebrow">A LITTLE INSPIRATION</p><h2 id="recipes-title">Your recipes <span class="count">{{ recipes.length }}</span></h2><p class="muted">Everything delicious, all in one place.</p></div><label class="search"><span class="sr-only">Search recipes</span><input v-model="search" type="search" placeholder="Search recipes..." /></label></div>
        <p v-if="loading" class="empty">Loading your recipes…</p>
        <div v-else-if="filtered.length === 0" class="empty"><span class="empty-icon">✳</span><h3>{{ search ? 'No recipes found' : 'Your collection starts here' }}</h3><p>{{ search ? 'Try another search.' : 'Add your first recipe and make this space your own.' }}</p></div>
        <div v-else class="grid"><article v-for="recipe in filtered" :key="recipe.id" class="recipe-card"><div class="card-top"><span class="recipe-mark">✳</span><span class="card-tag">RECIPE</span></div><h3>{{ recipe.title }}</h3><p class="description">{{ recipe.description || 'A recipe from your collection.' }}</p><details><summary>View recipe</summary><h4>Ingredients</h4><p class="multiline">{{ recipe.ingredients || 'No ingredients added.' }}</p><h4>Instructions</h4><p class="multiline">{{ recipe.instructions || 'No instructions added.' }}</p></details><div class="card-actions"><button class="link" @click="edit(recipe)">Edit</button><button class="link delete" @click="remove(recipe)">Delete</button></div></article></div>
      </section>
    </main><footer>YUMSTASH <span>Made for the meals worth remembering.</span></footer>
  </div>
</template>

<style>
:root { font-family: system-ui, sans-serif; color: #283b2d; background: #f8f7f1; font-synthesis: none; } * { box-sizing: border-box; } body { margin: 0; } button, input, textarea { font: inherit; } button { cursor: pointer; } .shell { max-width: 1440px; margin: auto; } .topbar { height: 76px; padding: 0 7%; display: flex; align-items: center; justify-content: space-between; border-bottom: 1px solid #e6e8de; } .brand { color: #294a35; text-decoration: none; font-weight: 800; font-size: 24px; letter-spacing: -1px; } .brand-icon { color: #ee9165; font-size: 28px; margin-right: 6px; } .top-label, .eyebrow, .card-tag { font-size: 11px; font-weight: 750; letter-spacing: .15em; color: #748979; } main { padding: 0 7%; } .hero { margin: 40px 0 75px; background: #e8efe4; border-radius: 16px; padding: 58px 7%; display: flex; justify-content: space-between; overflow: hidden; } .hero h1 { font: 500 clamp(40px,5vw,68px)/1.05 Georgia, serif; letter-spacing: -2px; margin: 20px 0; } .hero h1 em { color: #66836c; font-weight: normal; } .intro { line-height: 1.7; max-width: 420px; color: #627268; margin-bottom: 28px; } .primary { background: #315b40; border: 0; color: white; border-radius: 7px; padding: 14px 21px; font-weight: 650; } .primary:hover { background: #244832; } .primary:disabled { opacity: .6; } .hero-art { align-self: center; width: 260px; height: 260px; border: 1px solid #bdcdbb; border-radius: 50%; display: grid; place-items: center; position: relative; color: #719077; } .plate { border: 22px solid #f7f6e9; box-shadow: 0 8px 25px #718b7133; background: #d1dfcc; border-radius: 50%; width: 195px; height: 195px; display: grid; place-items: center; font-size: 90px; } .art-caption { position: absolute; right: -35px; bottom: 8px; color: #8a9f8d; font: italic 16px Georgia, serif; transform: rotate(-10deg); } .section-heading { display: flex; justify-content: space-between; align-items: end; gap: 20px; margin-bottom: 25px; } h2 { font: 500 36px Georgia, serif; margin: 8px 0; } .count { font: 600 13px system-ui; color: #6a846f; background: #e8efe4; padding: 5px 10px; border-radius: 20px; vertical-align: middle; } .muted { color: #849087; margin: 0; } input, textarea { width: 100%; border: 1px solid #dce2d8; border-radius: 7px; background: white; padding: 12px 14px; color: #283b2d; } input:focus, textarea:focus { outline: 2px solid #93ae98; outline-offset: 1px; } .search { width: 240px; } .grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(250px, 1fr)); gap: 20px; } .recipe-card, .form-card { background: white; border: 1px solid #e7eae2; border-radius: 12px; padding: 24px; } .form-card { margin-bottom: 65px; } .form-card form { display: grid; gap: 16px; } .form-card label { display: grid; gap: 7px; font-size: 14px; font-weight: 650; } .form-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 16px; } .form-card .primary { justify-self: start; } .card-top { display: flex; justify-content: space-between; align-items: center; } .recipe-mark { background: #edf3e8; color: #78967d; width: 46px; height: 46px; display: grid; place-items: center; border-radius: 12px; font-size: 28px; } .recipe-card h3 { font: 500 24px Georgia, serif; margin: 22px 0 8px; overflow-wrap: anywhere; } .description, .multiline { color: #748178; line-height: 1.6; overflow-wrap: anywhere; } .multiline { white-space: pre-wrap; } details { border-top: 1px solid #edf0e9; margin-top: 22px; padding-top: 16px; } summary { cursor: pointer; color: #315b40; font-weight: 650; } h4 { margin-bottom: 0; } .card-actions { display: flex; gap: 18px; margin-top: 20px; } .link { border: 0; background: transparent; color: #466f51; padding: 0; font-weight: 650; } .delete { color: #a45b50; } .empty { text-align: center; padding: 55px 20px; background: white; border: 1px dashed #d7e0d3; border-radius: 12px; color: #819087; } .empty h3 { font: 500 24px Georgia, serif; color: #405a46; } .empty-icon { font-size: 36px; color: #8baa8f; } .notice { background: #fff0eb; color: #8f4336; padding: 14px; border-radius: 8px; margin-bottom: 20px; } .notice .link { margin-left: 10px; } footer { margin: 80px 7% 0; padding: 25px 0; border-top: 1px solid #e6e8de; font-size: 11px; letter-spacing: .12em; color: #68806b; font-weight: 700; } footer span { float: right; letter-spacing: 0; color: #a0aaa1; font-weight: 400; } .sr-only { position: absolute; width: 1px; height: 1px; overflow: hidden; clip: rect(0,0,0,0); } @media(max-width: 700px) { .top-label, .hero-art { display: none; } .hero { padding: 38px 25px; margin-bottom: 50px; } .section-heading { align-items: stretch; flex-direction: column; } .search { width: 100%; } .form-grid { grid-template-columns: 1fr; } footer span { float: none; display: block; margin-top: 8px; } }
</style>
