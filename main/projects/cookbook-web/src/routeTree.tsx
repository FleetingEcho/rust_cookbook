import { createRootRoute, createRoute, Outlet } from '@tanstack/react-router'
import { RootLayout } from './components/RootLayout'
import { HomePage } from './pages/HomePage'
import { RecipesPage } from './pages/RecipesPage'
import { RecipeDetailPage } from './pages/RecipeDetailPage'
import { SearchPage } from './pages/SearchPage'
import { ByIngredientsPage } from './pages/ByIngredientsPage'
import { MealPlannerPage } from './pages/MealPlannerPage'
import { StatsPage } from './pages/StatsPage'

const rootRoute = createRootRoute({
  component: () => (
    <RootLayout>
      <Outlet />
    </RootLayout>
  ),
})

const indexRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/',
  component: HomePage,
})

const recipesRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/recipes',
  component: RecipesPage,
})

const recipeDetailRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/recipes/$id',
  component: RecipeDetailPage,
})

const searchRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/search',
  component: SearchPage,
})

const byIngredientsRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/by-ingredients',
  component: ByIngredientsPage,
})

const mealPlannerRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/meal-planner',
  component: MealPlannerPage,
})

const statsRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/stats',
  component: StatsPage,
})

export const routeTree = rootRoute.addChildren([
  indexRoute,
  recipesRoute,
  recipeDetailRoute,
  searchRoute,
  byIngredientsRoute,
  mealPlannerRoute,
  statsRoute,
])
