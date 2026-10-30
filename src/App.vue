<script setup>
import { computed, onMounted, ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { open } from '@tauri-apps/plugin-dialog'
import packageMetadata from '../package.json'

const translations = {
  en: {
    appSubtitle: 'DESKTOP SETUP', headerCaption: 'SETUP ASSISTANT', settingsButton: 'Settings', closeSettings: 'Back to setup',
    configAria: 'Yabridge setup', stepsAria: 'Setup steps', stepWord: 'STEP', steps: ['Requirements', 'Locations', 'Sync'],
    stepSystem: 'SYSTEM', requirementsTitle: 'System requirements', requirementsDescription: 'Check the components needed to run Windows plugins on Linux.',
    detectedDistro: 'DETECTED DISTRIBUTION', detectingDistro: 'Detecting Linux distribution...', systemUnavailable: 'System unavailable', refresh: 'Refresh', checking: 'Checking', available: 'Available', missing: 'Missing', unavailable: 'Unavailable',
    wine: 'Wine Staging', wineDescription: 'Compatibility layer for Windows plugins', yabridge: 'Yabridge', yabridgeDescription: 'Bridge and yabridgectl for managing plugins',
    dependencyError: 'Could not query the system. Open the Tauri desktop app and try again.', fedoraPlan: 'Fedora: enable the community patrickl/wine-tkg COPR recommended by Yabridge to install Wine Staging and Yabridge.', archPlan: 'Arch/Manjaro: install wine-staging, yabridge, and yabridgectl from your configured pacman repositories.', fedoraAtomicPlan: 'Automatic installation is disabled on Fedora Atomic systems. Follow your distribution’s rpm-ostree instructions.', fedoraUnsupportedPlan: 'Automatic installation is available only on Fedora 43/44 x86_64 non-Atomic systems.', debianPlan: 'The Yabridge documentation recommends Wine Staging from WineHQ on Debian-based systems. Install the compatible packages using the official instructions.', unsupportedPlan: 'There is no verified automatic installation path for this distribution. See the official Yabridge instructions.',
    installDependencies: 'Install dependencies', installing: 'Installing...', officialInstructions: 'Official instructions', readyMessage: 'All requirements are available. You can continue.',
    authorizeInstall: 'Authorize installation?', passwordNotice: 'The package manager will request your password. Fedora will enable a community COPR and may install or replace system packages.', cancel: 'Cancel', continue: 'Continue',
    stepLibrary: 'LIBRARY', locationsTitle: 'Plugin locations', locationsDescription: 'Add folders where you installed Windows plugins so Yabridge can manage them.', registeredFolders: 'REGISTERED FOLDERS', pluginLibrary: 'Plugin library', updateList: 'Refresh list', updating: 'Refreshing...', pluginFolder: 'Plugin installation folder', pathPlaceholder: 'Choose a folder or enter its full path', chooseFolder: 'Choose folder', addLocation: 'Add location', removeLink: 'Remove link', removeNotice: 'Removing a link does not delete files. Run Sync after changing folders.', listLoading: 'Loading plugin locations...', listError: 'Could not list locations', chooseLocationFirst: 'Choose or enter a folder first.', pickerError: 'Could not open folder picker', commandError: 'yabridgectl {action} failed', syncAfterChanges: 'Run Sync to apply the changes.',
    stepFinish: 'FINISH', syncTitle: 'Sync plugins', syncDescription: 'Check Yabridge status or apply changes to your registered plugin folders.', status: 'Status', sync: 'Sync', commandOutput: 'COMMAND OUTPUT', terminal: 'TERMINAL', commandReady: 'Ready. Run Status or Sync.', commandRunning: 'Running yabridgectl', commandNoOutput: 'Command completed with no output.', error: 'Error',
    back: 'Back', stepCount: 'STEP {step} OF 3', setupReady: 'SETUP COMPLETE',
    settingsEyebrow: 'PREFERENCES', settingsTitle: 'Settings', settingsDescription: 'Choose the language used throughout the application.', language: 'Application language', aboutEyebrow: 'APPLICATION', aboutTitle: 'About', aboutDescription: 'A desktop utility for managing Windows plugins with Yabridge.', version: 'Version',
    installAuthWait: 'Waiting for system authorization...',
  },
  es: {
    appSubtitle: 'CONFIGURACIÓN DE ESCRITORIO', headerCaption: 'ASISTENTE DE CONFIGURACIÓN', settingsButton: 'Configuración', closeSettings: 'Volver al asistente',
    configAria: 'Configuración de Yabridge', stepsAria: 'Pasos de configuración', stepWord: 'PASO', steps: ['Requisitos', 'Ubicaciones', 'Sincronización'],
    stepSystem: 'SISTEMA', requirementsTitle: 'Requisitos del sistema', requirementsDescription: 'Comprueba los componentes necesarios para ejecutar plugins de Windows en Linux.',
    detectedDistro: 'DISTRIBUCIÓN DETECTADA', detectingDistro: 'Detectando distribución Linux...', systemUnavailable: 'Sistema no disponible', refresh: 'Actualizar', checking: 'Comprobando', available: 'Disponible', missing: 'No encontrado', unavailable: 'No disponible',
    wine: 'Wine Staging', wineDescription: 'Capa de compatibilidad para plugins de Windows', yabridge: 'Yabridge', yabridgeDescription: 'Bridge y yabridgectl para gestionar plugins',
    dependencyError: 'No se pudo consultar el sistema. Abre la aplicación de escritorio Tauri e inténtalo de nuevo.', fedoraPlan: 'Fedora: habilita el COPR comunitario patrickl/wine-tkg recomendado por Yabridge para instalar Wine Staging y Yabridge.', archPlan: 'Arch/Manjaro: instala wine-staging, yabridge y yabridgectl desde los repositorios pacman configurados.', fedoraAtomicPlan: 'La instalación automática está deshabilitada en Fedora Atomic. Sigue las instrucciones rpm-ostree de tu distribución.', fedoraUnsupportedPlan: 'La instalación automática solo está disponible en Fedora 43/44 x86_64 no Atomic.', debianPlan: 'La documentación de Yabridge recomienda Wine Staging de WineHQ en sistemas basados en Debian. Instala los paquetes compatibles siguiendo las instrucciones oficiales.', unsupportedPlan: 'No hay una ruta de instalación automática verificada para esta distribución. Consulta las instrucciones oficiales de Yabridge.',
    installDependencies: 'Instalar dependencias', installing: 'Instalando...', officialInstructions: 'Instrucciones oficiales', readyMessage: 'Todos los requisitos están disponibles. Puedes continuar.',
    authorizeInstall: '¿Autorizar la instalación?', passwordNotice: 'El gestor de paquetes solicitará tu contraseña. Fedora habilitará un COPR comunitario y podría instalar o sustituir paquetes del sistema.', cancel: 'Cancelar', continue: 'Continuar',
    stepLibrary: 'BIBLIOTECA', locationsTitle: 'Ubicaciones de plugins', locationsDescription: 'Añade las carpetas donde instalaste plugins de Windows para que Yabridge pueda gestionarlas.', registeredFolders: 'CARPETAS REGISTRADAS', pluginLibrary: 'Biblioteca de plugins', updateList: 'Actualizar lista', updating: 'Actualizando...', pluginFolder: 'Carpeta de instalación del plugin', pathPlaceholder: 'Elige una carpeta o introduce la ruta completa', chooseFolder: 'Elegir carpeta', addLocation: 'Añadir ubicación', removeLink: 'Quitar vínculo', removeNotice: 'Quitar el vínculo no elimina los archivos. Ejecuta Sync después de cambiar las carpetas.', listLoading: 'Cargando ubicaciones de plugins...', listError: 'No se pudieron listar las ubicaciones', chooseLocationFirst: 'Elige o introduce una carpeta primero.', pickerError: 'No se pudo abrir el selector de carpetas', commandError: 'Error en yabridgectl {action}', syncAfterChanges: 'Ejecuta Sync para aplicar los cambios.',
    stepFinish: 'FINALIZAR', syncTitle: 'Sincronizar plugins', syncDescription: 'Consulta el estado de Yabridge o aplica los cambios a las carpetas registradas.', status: 'Estado', sync: 'Sync', commandOutput: 'SALIDA DEL COMANDO', terminal: 'TERMINAL', commandReady: 'Listo. Ejecuta Estado o Sync.', commandRunning: 'Ejecutando yabridgectl', commandNoOutput: 'El comando finalizó sin salida.', error: 'Error',
    back: 'Atrás', stepCount: 'PASO {step} DE 3', setupReady: 'CONFIGURACIÓN COMPLETA',
    settingsEyebrow: 'PREFERENCIAS', settingsTitle: 'Configuración', settingsDescription: 'Elige el idioma de la aplicación.', language: 'Idioma de la aplicación', aboutEyebrow: 'APLICACIÓN', aboutTitle: 'Acerca de', aboutDescription: 'Una utilidad de escritorio para gestionar plugins de Windows con Yabridge.', version: 'Versión',
    installAuthWait: 'Esperando autorización del sistema...',
  },
  pt: {
    appSubtitle: 'CONFIGURAÇÃO DESKTOP', headerCaption: 'ASSISTENTE DE CONFIGURAÇÃO', settingsButton: 'Configurações', closeSettings: 'Voltar ao assistente',
    configAria: 'Configuração do Yabridge', stepsAria: 'Etapas da configuração', stepWord: 'ETAPA', steps: ['Requisitos', 'Locais', 'Sincronização'],
    stepSystem: 'SISTEMA', requirementsTitle: 'Requisitos do sistema', requirementsDescription: 'Verifique os componentes necessários para executar plugins Windows no Linux.',
    detectedDistro: 'DISTRIBUIÇÃO DETECTADA', detectingDistro: 'Detectando distribuição Linux...', systemUnavailable: 'Sistema indisponível', refresh: 'Atualizar', checking: 'Verificando', available: 'Disponível', missing: 'Ausente', unavailable: 'Indisponível',
    wine: 'Wine Staging', wineDescription: 'Camada de compatibilidade para plugins Windows', yabridge: 'Yabridge', yabridgeDescription: 'Bridge e yabridgectl para gerenciar plugins',
    dependencyError: 'Não foi possível consultar o sistema. Abra o aplicativo desktop Tauri e tente novamente.', fedoraPlan: 'Fedora: habilita o COPR comunitário patrickl/wine-tkg recomendado pelo Yabridge para instalar Wine Staging e Yabridge.', archPlan: 'Arch/Manjaro: instala wine-staging, yabridge e yabridgectl dos repositórios pacman configurados.', fedoraAtomicPlan: 'A instalação automática está desabilitada no Fedora Atomic. Siga as instruções rpm-ostree da sua distribuição.', fedoraUnsupportedPlan: 'A instalação automática está disponível apenas no Fedora 43/44 x86_64 não Atomic.', debianPlan: 'A documentação do Yabridge recomenda Wine Staging do WineHQ em sistemas baseados em Debian. Instale os pacotes compatíveis seguindo as instruções oficiais.', unsupportedPlan: 'Não há uma rota de instalação automática verificada para esta distribuição. Consulte as instruções oficiais do Yabridge.',
    installDependencies: 'Instalar dependências', installing: 'Instalando...', officialInstructions: 'Instruções oficiais', readyMessage: 'Todos os requisitos estão disponíveis. Você pode continuar.',
    authorizeInstall: 'Autorizar instalação?', passwordNotice: 'O gerenciador de pacotes solicitará sua senha. No Fedora, será habilitado um COPR comunitário e pacotes poderão ser instalados ou substituídos.', cancel: 'Cancelar', continue: 'Continuar',
    stepLibrary: 'BIBLIOTECA', locationsTitle: 'Locais de plugins', locationsDescription: 'Adicione as pastas onde instalou plugins Windows para que o Yabridge possa gerenciá-las.', registeredFolders: 'PASTAS REGISTRADAS', pluginLibrary: 'Biblioteca de plugins', updateList: 'Atualizar lista', updating: 'Atualizando...', pluginFolder: 'Pasta de instalação do plugin', pathPlaceholder: 'Escolha uma pasta ou informe o caminho completo', chooseFolder: 'Escolher pasta', addLocation: 'Adicionar local', removeLink: 'Remover vínculo', removeNotice: 'Remover o vínculo não apaga os arquivos. Execute Sync após alterar as pastas.', listLoading: 'Carregando locais de plugins...', listError: 'Não foi possível listar os locais', chooseLocationFirst: 'Escolha ou informe uma pasta primeiro.', pickerError: 'Não foi possível abrir o seletor de pastas', commandError: 'Falha em yabridgectl {action}', syncAfterChanges: 'Execute Sync para aplicar as alterações.',
    stepFinish: 'CONCLUSÃO', syncTitle: 'Sincronizar plugins', syncDescription: 'Consulte o estado do Yabridge ou aplique alterações às pastas registradas.', status: 'Status', sync: 'Sincronizar', commandOutput: 'SAÍDA DO COMANDO', terminal: 'TERMINAL', commandReady: 'Pronto. Execute Status ou Sincronizar.', commandRunning: 'Executando yabridgectl', commandNoOutput: 'Comando concluído sem saída.', error: 'Erro',
    back: 'Voltar', stepCount: 'ETAPA {step} DE 3', setupReady: 'CONFIGURAÇÃO CONCLUÍDA',
    settingsEyebrow: 'PREFERÊNCIAS', settingsTitle: 'Configurações', settingsDescription: 'Escolha o idioma usado no aplicativo.', language: 'Idioma do aplicativo', aboutEyebrow: 'APLICATIVO', aboutTitle: 'Sobre', aboutDescription: 'Um utilitário desktop para gerenciar plugins Windows com Yabridge.', version: 'Versão',
    installAuthWait: 'Aguardando autorização do sistema...',
  },
}

function idiomaGuardado() {
  try {
    const savedLanguage = localStorage.getItem('yabridge-language')
    return savedLanguage === 'es' || savedLanguage === 'pt' ? savedLanguage : 'en'
  } catch {
    return 'en'
  }
}

const language = ref(idiomaGuardado())
const t = computed(() => translations[language.value])
const appVersion = packageMetadata.version
const isSettings = ref(false)
const logOutput = ref('')
const isRunning = ref(false)
const activeCommand = ref('')
const dependencyStatus = ref(null)
const dependencyError = ref('')
const isDetectingDependencies = ref(true)
const isInstallingDependencies = ref(false)
const showInstallConfirmation = ref(false)
const installationOutput = ref('')
const pluginPath = ref('')
const pluginLocationsOutput = ref('Carregando locais de plugins...')
const isManagingPluginLocations = ref(false)
const currentStep = ref(0)
const steps = computed(() => [t.value.steps[0], t.value.steps[1], t.value.steps[2]])
const installationPlanText = computed(() => {
  if (!dependencyStatus.value) return ''
  const distribution = dependencyStatus.value.distribution.toLowerCase()
  if (dependencyStatus.value.automatic_install_supported) {
    return distribution.includes('fedora') ? t.value.fedoraPlan : t.value.archPlan
  }
  if (distribution.includes('atomic')) return t.value.fedoraAtomicPlan
  if (distribution.includes('fedora')) return t.value.fedoraUnsupportedPlan
  if (['debian', 'ubuntu', 'mint', 'pop!'].some((name) => distribution.includes(name))) return t.value.debianPlan
  return t.value.unsupportedPlan
})
const requirementsReady = computed(() => Boolean(
  dependencyStatus.value?.wine_installed && dependencyStatus.value?.yabridge_installed,
))

watch(language, (value) => {
  if (typeof document !== 'undefined') document.documentElement.lang = value
  try {
    localStorage.setItem('yabridge-language', value)
  } catch {
    return
  }
}, { immediate: true })

function abrirEtapa(indice) {
  currentStep.value = indice
}

function avancarEtapa() {
  if (currentStep.value < steps.value.length - 1) currentStep.value += 1
}

function voltarEtapa() {
  if (currentStep.value > 0) currentStep.value -= 1
}

onMounted(() => {
  void carregarDependencias()
  void listarLocaisPlugins()
})

async function carregarDependencias() {
  isDetectingDependencies.value = true
  dependencyError.value = ''

  try {
    dependencyStatus.value = await invoke('detectar_dependencias')
  } catch (erro) {
    dependencyError.value = t.value.dependencyError
  } finally {
    isDetectingDependencies.value = false
  }
}

async function instalarDependencias() {
  if (!dependencyStatus.value?.automatic_install_supported || isInstallingDependencies.value) return

  showInstallConfirmation.value = false
  isInstallingDependencies.value = true
  installationOutput.value = t.value.installAuthWait
  let pararDeOuvir

  try {
    pararDeOuvir = await listen('installation-progress', (event) => {
      installationOutput.value += `\n${event.payload}`
    })
    const resultado = await invoke('instalar_dependencias')
    installationOutput.value += `\n${resultado}`
    await carregarDependencias()
  } catch (erro) {
    installationOutput.value = `${t.value.error}: ${erro}`
  } finally {
    pararDeOuvir?.()
    isInstallingDependencies.value = false
  }
}

async function rodarYabridge(acao) {
  if (isRunning.value) return

  isRunning.value = true
  activeCommand.value = acao
  logOutput.value = `${t.value.commandRunning} yabridgectl ${acao}...`

  try {
    const resposta = await invoke('executar_yabridge', { comando: acao })
    logOutput.value = resposta || t.value.commandNoOutput
  } catch (erro) {
    logOutput.value = `${t.value.error}: ${erro}`
  } finally {
    isRunning.value = false
    activeCommand.value = ''
  }
}

async function listarLocaisPlugins() {
  isManagingPluginLocations.value = true
  pluginLocationsOutput.value = `${t.value.commandRunning} yabridgectl list...`

  try {
    pluginLocationsOutput.value = await invoke('gerenciar_locais_plugins', {
      acao: 'list',
      caminho: null,
    })
  } catch (erro) {
    pluginLocationsOutput.value = `${t.value.listError}: ${erro}`
  } finally {
    isManagingPluginLocations.value = false
  }
}

async function selecionarPastaPlugin() {
  try {
    const selecionado = await open({
      directory: true,
      multiple: false,
      title: t.value.pluginFolder,
    })
    if (typeof selecionado === 'string') pluginPath.value = selecionado
  } catch (erro) {
    pluginLocationsOutput.value = `${t.value.pickerError}: ${erro}`
  }
}

async function gerenciarLocalPlugin(acao) {
  if (isManagingPluginLocations.value) return

  const caminho = pluginPath.value.trim()
  if ((acao === 'add' || acao === 'rm') && !caminho) {
    pluginLocationsOutput.value = t.value.chooseLocationFirst
    return
  }

  isManagingPluginLocations.value = true
  pluginLocationsOutput.value = `${t.value.commandRunning} yabridgectl ${acao}...`

  try {
    const resultado = await invoke('gerenciar_locais_plugins', {
      acao,
      caminho: acao === 'add' || acao === 'rm' ? caminho : null,
    })

    if (acao === 'add' || acao === 'rm') {
      const locais = await invoke('gerenciar_locais_plugins', {
        acao: 'list',
        caminho: null,
      })
      pluginPath.value = ''
      pluginLocationsOutput.value = `${resultado}\n\n${locais}\n\n${t.value.syncAfterChanges}`
    } else {
      pluginLocationsOutput.value = resultado
    }
  } catch (erro) {
    pluginLocationsOutput.value = `${t.value.commandError.replace('{action}', acao)}: ${erro}`
  } finally {
    isManagingPluginLocations.value = false
  }
}
</script>

<template>
  <main class="app-shell wizard-shell">
    <header class="topbar">
      <span class="brand-mark" aria-hidden="true">Y</span>
      <div class="brand-copy">
        <span class="brand-name">Yabridge</span>
        <span class="brand-subtitle">{{ t.appSubtitle }}</span>
      </div>
      <span class="topbar-caption">{{ isSettings ? t.settingsEyebrow : t.headerCaption }}</span>
      <button class="settings-trigger" type="button" @click="isSettings = !isSettings">
        <span class="settings-icon" aria-hidden="true">⚙</span>
        {{ isSettings ? t.closeSettings : t.settingsButton }}
      </button>
    </header>

    <section class="workspace wizard-workspace" :class="{ 'settings-mode': isSettings }" :aria-label="t.configAria">
      <template v-if="!isSettings">
      <nav class="stepper" :aria-label="t.stepsAria">
        <button
          v-for="(step, indice) in steps"
          :key="step"
          class="step-item"
          :class="{ active: currentStep === indice, complete: currentStep > indice }"
          :aria-current="currentStep === indice ? 'step' : undefined"
          type="button"
          @click="abrirEtapa(indice)"
        >
          <span class="step-number">{{ currentStep > indice ? '&#10003;' : `0${indice + 1}` }}</span>
          <span class="step-label">{{ step }}</span>
        </button>
      </nav>

      <div v-if="currentStep === 0" class="wizard-page" key="requirements">
        <div class="step-heading">
          <p class="eyebrow">{{ t.stepWord }} 01 <span>/</span> {{ t.stepSystem }}</p>
          <h1>{{ t.requirementsTitle }}</h1>
          <p class="step-description">{{ t.requirementsDescription }}</p>
        </div>

        <section class="step-surface requirement-surface" :aria-label="t.requirementsTitle" aria-live="polite">
          <div class="system-summary">
            <div>
              <span class="summary-label">{{ t.detectedDistro }}</span>
              <strong>{{ isDetectingDependencies ? t.detectingDistro : dependencyStatus?.distribution || t.systemUnavailable }}</strong>
            </div>
            <button
              class="refresh-button"
              type="button"
              :disabled="isDetectingDependencies || isInstallingDependencies"
              @click="carregarDependencias"
            >
              {{ isDetectingDependencies ? `${t.checking}...` : t.refresh }}
            </button>
          </div>

          <div class="requirement-list">
            <div class="requirement-row" :class="{ missing: !dependencyStatus?.wine_installed }">
                <span class="requirement-icon" :class="{ unknown: !isDetectingDependencies && !dependencyStatus }" aria-hidden="true">{{ isDetectingDependencies ? '…' : !dependencyStatus ? '?' : dependencyStatus.wine_installed ? '✓' : '!' }}</span>
              <span class="requirement-copy">
                <strong>{{ t.wine }}</strong>
                <small>{{ t.wineDescription }}</small>
              </span>
              <span class="requirement-status" :class="{ unknown: !isDetectingDependencies && !dependencyStatus, missing: dependencyStatus && !dependencyStatus.wine_installed }">
                {{ isDetectingDependencies ? t.checking : !dependencyStatus ? t.unavailable : dependencyStatus.wine_installed ? t.available : t.missing }}
              </span>
            </div>
            <div class="requirement-row" :class="{ missing: !dependencyStatus?.yabridge_installed }">
                <span class="requirement-icon" :class="{ unknown: !isDetectingDependencies && !dependencyStatus }" aria-hidden="true">{{ isDetectingDependencies ? '…' : !dependencyStatus ? '?' : dependencyStatus.yabridge_installed ? '✓' : '!' }}</span>
              <span class="requirement-copy">
                <strong>{{ t.yabridge }}</strong>
                <small>{{ t.yabridgeDescription }}</small>
              </span>
              <span class="requirement-status" :class="{ unknown: !isDetectingDependencies && !dependencyStatus, missing: dependencyStatus && !dependencyStatus.yabridge_installed }">
                {{ isDetectingDependencies ? t.checking : !dependencyStatus ? t.unavailable : dependencyStatus.yabridge_installed ? t.available : t.missing }}
              </span>
            </div>
          </div>

          <p v-if="dependencyError" class="system-message error-message">{{ t.dependencyError }}</p>
          <p v-else-if="dependencyStatus" class="system-message">{{ installationPlanText }}</p>

          <div v-if="dependencyStatus && (!dependencyStatus.wine_installed || !dependencyStatus.yabridge_installed)" class="requirement-actions">
            <button
              v-if="dependencyStatus.automatic_install_supported"
              class="install-button"
              type="button"
              :disabled="isInstallingDependencies"
              @click="showInstallConfirmation = true"
            >
              {{ isInstallingDependencies ? t.installing : t.installDependencies }}
            </button>
            <a class="documentation-link" :href="dependencyStatus.documentation_url" target="_blank" rel="noreferrer">
              {{ t.officialInstructions }}
            </a>
          </div>

          <p v-else-if="requirementsReady" class="ready-message">{{ t.readyMessage }}</p>

          <div v-if="showInstallConfirmation" class="install-confirmation" role="dialog" aria-modal="true" aria-labelledby="install-confirmation-title">
            <h3 id="install-confirmation-title">{{ t.authorizeInstall }}</h3>
            <p>{{ installationPlanText }}</p>
            <p>{{ t.passwordNotice }}</p>
            <div class="confirmation-actions">
              <button class="cancel-button" type="button" @click="showInstallConfirmation = false">{{ t.cancel }}</button>
              <button class="install-button" type="button" @click="instalarDependencias">{{ t.continue }}</button>
            </div>
          </div>

          <pre v-if="installationOutput" class="installation-output" aria-live="polite">{{ installationOutput }}</pre>
        </section>
      </div>

      <div v-else-if="currentStep === 1" class="wizard-page" key="locations">
        <div class="step-heading">
          <p class="eyebrow">{{ t.stepWord }} 02 <span>/</span> {{ t.stepLibrary }}</p>
          <h1>{{ t.locationsTitle }}</h1>
          <p class="step-description">{{ t.locationsDescription }}</p>
        </div>

        <section class="step-surface plugin-manager" aria-labelledby="plugin-manager-title">
          <div class="plugin-manager-heading">
            <div>
              <span class="summary-label">{{ t.registeredFolders }}</span>
              <h2 id="plugin-manager-title">{{ t.pluginLibrary }}</h2>
            </div>
            <button
              class="refresh-button"
              type="button"
              :disabled="isManagingPluginLocations"
              @click="gerenciarLocalPlugin('list')"
            >
              {{ isManagingPluginLocations ? t.updating : t.updateList }}
            </button>
          </div>

          <label class="path-label" for="plugin-path">{{ t.pluginFolder }}</label>
          <div class="path-control">
            <input
              id="plugin-path"
              v-model="pluginPath"
              type="text"
              :placeholder="t.pathPlaceholder"
              :disabled="isManagingPluginLocations"
            />
            <button
              class="browse-button"
              type="button"
              :disabled="isManagingPluginLocations"
              @click="selecionarPastaPlugin"
            >
              {{ t.chooseFolder }}
            </button>
          </div>
          <div class="location-actions">
            <button
              class="install-button"
              type="button"
              :disabled="isManagingPluginLocations || !pluginPath.trim()"
              @click="gerenciarLocalPlugin('add')"
            >
              {{ t.addLocation }}
            </button>
            <button
              class="remove-location-button"
              type="button"
              :disabled="isManagingPluginLocations || !pluginPath.trim()"
              @click="gerenciarLocalPlugin('rm')"
            >
              {{ t.removeLink }}
            </button>
          </div>
          <p class="manager-note">{{ t.removeNotice }}</p>
          <pre class="locations-output" aria-live="polite">{{ pluginLocationsOutput }}</pre>
        </section>
      </div>

      <div v-else class="wizard-page" key="sync">
        <div class="step-heading">
          <p class="eyebrow">{{ t.stepWord }} 03 <span>/</span> {{ t.stepFinish }}</p>
          <h1>{{ t.syncTitle }}</h1>
          <p class="step-description">{{ t.syncDescription }}</p>
        </div>

        <div class="actions" :aria-label="t.syncTitle">
          <button
            class="action-button status-button"
            type="button"
            :disabled="isRunning || isInstallingDependencies"
            @click="rodarYabridge('status')"
          >
            <span class="button-indicator" aria-hidden="true">01</span>
            <span>{{ t.status }}</span>
            <span class="button-arrow" aria-hidden="true">&rarr;</span>
          </button>
          <button
            class="action-button sync-button"
            type="button"
            :disabled="isRunning || isInstallingDependencies"
            @click="rodarYabridge('sync')"
          >
            <span class="button-indicator" aria-hidden="true">02</span>
            <span>{{ t.sync }}</span>
            <span class="button-arrow" aria-hidden="true">&rarr;</span>
          </button>
        </div>

        <section class="terminal" aria-labelledby="terminal-title">
          <header class="terminal-header">
            <div class="terminal-title-group">
              <span class="terminal-lights" aria-hidden="true"><i></i><i></i><i></i></span>
              <h2 id="terminal-title">{{ t.commandOutput }}</h2>
            </div>
            <span class="terminal-command">{{ activeCommand ? `yabridgectl ${activeCommand}` : t.terminal }}</span>
          </header>
          <pre class="terminal-output" aria-live="polite">{{ logOutput || t.commandReady }}</pre>
        </section>
      </div>

      <footer class="wizard-footer">
        <button class="back-button" type="button" :disabled="currentStep === 0" @click="voltarEtapa">
          {{ t.back }}
        </button>
        <span class="step-counter">{{ t.stepCount.replace('{step}', `0${currentStep + 1}`) }}</span>
        <button
          v-if="currentStep < steps.length - 1"
          class="continue-button"
          type="button"
          @click="avancarEtapa"
        >
          {{ t.continue }} <span aria-hidden="true">&rarr;</span>
        </button>
        <span v-else class="complete-label">{{ t.setupReady }}</span>
      </footer>
      </template>

      <section v-else class="settings-page" aria-labelledby="settings-title">
        <div class="step-heading">
          <p class="eyebrow">{{ t.settingsEyebrow }}</p>
          <h1 id="settings-title">{{ t.settingsTitle }}</h1>
          <p class="step-description">{{ t.settingsDescription }}</p>
        </div>

        <section class="settings-card" aria-labelledby="language-title">
          <div>
            <p class="settings-card-eyebrow">{{ t.settingsEyebrow }}</p>
            <h2 id="language-title">{{ t.language }}</h2>
          </div>
          <label class="language-field" for="language-select">
            <span>{{ t.language }}</span>
            <select id="language-select" v-model="language">
              <option value="en">English</option>
              <option value="es">Español</option>
              <option value="pt">Português (Brasil)</option>
            </select>
          </label>
        </section>

        <section class="settings-card about-card" aria-labelledby="about-title">
          <div>
            <p class="settings-card-eyebrow">{{ t.aboutEyebrow }}</p>
            <h2 id="about-title">{{ t.aboutTitle }}</h2>
            <p class="about-description">{{ t.aboutDescription }}</p>
          </div>
          <div class="version-badge">
            <span>{{ t.version }}</span>
            <strong>v{{ appVersion }}</strong>
          </div>
        </section>
      </section>
    </section>
  </main>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

</style>

<style scoped>
.wizard-shell {
  display: flex;
  min-height: 100vh;
  flex-direction: column;
}

.wizard-shell .topbar {
  flex: 0 0 66px;
  gap: 12px;
  padding: 0 max(24px, calc((100% - 920px) / 2));
}

.brand-copy {
  display: flex;
  flex-direction: column;
  gap: 1px;
}

.brand-subtitle {
  color: #819084;
  font-size: 9px;
  font-weight: 650;
  letter-spacing: 1px;
}

.app-shell.wizard-shell .wizard-workspace {
  width: min(calc(100% - 48px), 680px);
  height: calc(100vh - 66px);
  min-height: 0;
  display: grid;
  grid-template-rows: 48px minmax(0, 1fr) 58px;
  gap: 14px;
  margin: 0 auto;
  padding: 21px 0 16px;
}

.stepper {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 8px;
}

.step-item {
  min-width: 0;
  display: flex;
  align-items: center;
  gap: 9px;
  padding: 0 11px;
  border: 1px solid #303d34;
  border-radius: 6px;
  background: #171e1a;
  color: #91a094;
  text-align: left;
  font: inherit;
  cursor: pointer;
}

.step-item:disabled {
  cursor: not-allowed;
  opacity: 0.55;
}

.step-number {
  width: 25px;
  height: 25px;
  flex: 0 0 25px;
  display: grid;
  place-items: center;
  border: 1px solid #435146;
  border-radius: 50%;
  color: #a9b5aa;
  font-family: "SFMono-Regular", Consolas, monospace;
  font-size: 10px;
}

.step-label {
  overflow: hidden;
  font-size: 11px;
  font-weight: 650;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.step-item.active {
  border-color: #687f4b;
  background: #1d2a20;
  color: #eff8e9;
}

.step-item.active .step-number,
.step-item.complete .step-number {
  border-color: #9cbd64;
  background: #31452b;
  color: #d7f0a8;
}

.step-item.complete {
  color: #bbcca9;
}

.wizard-page {
  min-height: 0;
  overflow: auto;
  padding: 4px 3px 8px;
  animation: page-arrive 180ms ease-out;
}

@keyframes page-arrive {
  from { opacity: 0.72; transform: translateY(4px); }
  to { opacity: 1; transform: translateY(0); }
}

.step-heading {
  margin: 0 0 17px;
}

.step-heading .eyebrow {
  margin: 0 0 7px;
  color: #a8c879;
  font-size: 9px;
  font-weight: 750;
  letter-spacing: 1.15px;
}

.step-heading .eyebrow span {
  padding: 0 4px;
  color: #637164;
}

.step-heading h1 {
  margin: 0;
  color: #e8eee8;
  font-size: 24px;
  font-weight: 680;
  line-height: 1.2;
  text-align: left;
}

.step-description {
  max-width: 620px;
  margin: 7px 0 0;
  color: #9aa79b;
  font-size: 12px;
  line-height: 1.55;
}

.step-surface {
  width: 100%;
  max-width: 640px;
  margin: 0 auto;
  padding: 16px;
  border: 1px solid #303d34;
  border-radius: 7px;
  background: #171e1a;
  box-shadow: 0 12px 30px rgba(0, 0, 0, 0.15);
}

.system-summary,
.plugin-manager-heading {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.system-summary {
  margin-bottom: 12px;
  padding: 0 1px 12px;
  border-bottom: 1px solid #303d34;
}

.summary-label {
  display: block;
  margin-bottom: 4px;
  color: #819084;
  font-size: 8px;
  font-weight: 750;
  letter-spacing: 1px;
}

.system-summary strong {
  color: #e0e8e0;
  font-size: 12px;
  font-weight: 600;
}

.requirement-list {
  overflow: hidden;
  border: 1px solid #303d34;
  border-radius: 5px;
  background: #1c241f;
}

.requirement-row {
  min-height: 57px;
  display: flex;
  align-items: center;
  gap: 11px;
  padding: 8px 11px;
}

.requirement-row + .requirement-row {
  border-top: 1px solid #303d34;
}

.requirement-icon {
  width: 22px;
  height: 22px;
  flex: 0 0 22px;
  display: grid;
  place-items: center;
  border-radius: 50%;
  background: #243a2d;
  color: #b6e582;
  font-size: 12px;
  font-weight: 800;
}

.requirement-row.missing .requirement-icon {
  background: #493126;
  color: #ffb389;
}

.requirement-copy {
  min-width: 0;
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 3px;
}

.requirement-copy strong {
  color: #e5ece5;
  font-size: 12px;
  font-weight: 650;
}

.requirement-copy small {
  overflow: hidden;
  color: #8d9a8f;
  font-size: 10px;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.requirement-status {
  color: #b6e582;
  font-size: 10px;
  white-space: nowrap;
}

.requirement-row.missing .requirement-status {
  color: #ffb389;
}

.system-message {
  margin: 12px 1px 0;
  color: #99a69a;
  font-size: 10px;
  line-height: 1.55;
}

.error-message {
  color: #ffb389;
}

.requirement-actions {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 14px;
  margin-top: 12px;
}

.ready-message {
  margin: 12px 1px 0;
  color: #b6e582;
  font-size: 11px;
}

.step-surface .plugin-manager-heading {
  margin-bottom: 14px;
}

.plugin-manager-heading h2 {
  margin: 0;
  color: #e5ece5;
  font-size: 14px;
}

.step-surface .manager-note {
  margin-top: 11px;
  font-size: 9px;
}

.step-surface .locations-output {
  max-height: 175px;
  margin-top: 10px;
  border: 1px solid #303d34;
}

.wizard-page .actions {
  margin: 0 0 14px;
}

.wizard-page .action-button {
  min-height: 58px;
}

.wizard-page .terminal {
  max-width: 640px;
  margin: 0 auto;
}

.wizard-page .terminal-output {
  min-height: 175px;
  max-height: 260px;
}

.wizard-footer {
  display: grid;
  grid-template-columns: 1fr auto 1fr;
  align-items: center;
  gap: 12px;
  padding-top: 11px;
  border-top: 1px solid #2c3830;
}

.back-button,
.continue-button {
  min-width: 110px;
  min-height: 40px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 12px;
  padding: 0 16px;
  border: 1px solid #3a473d;
  border-radius: 5px;
  box-shadow: none;
  font: inherit;
  font-size: 11px;
  font-weight: 700;
  cursor: pointer;
}

.back-button {
  justify-self: start;
  background: #202923;
  color: #d5ded5;
}

.back-button:hover:not(:disabled) {
  border-color: #596c5c;
  background: #29362d;
}

.back-button:disabled {
  cursor: default;
  opacity: 0.38;
}

.continue-button {
  justify-self: end;
  border-color: #789c4c;
  background: #476032;
  color: #f3fae7;
}

.continue-button:hover:not(:disabled) {
  border-color: #a1c66c;
  background: #58783b;
}

.continue-button:disabled {
  border-color: #39433a;
  background: #29312b;
  color: #788278;
  cursor: not-allowed;
}

.step-counter,
.complete-label {
  color: #859287;
  font-family: "SFMono-Regular", Consolas, monospace;
  font-size: 9px;
  font-weight: 650;
  letter-spacing: 0.7px;
  white-space: nowrap;
}

.complete-label {
  justify-self: end;
  color: #b6e582;
}

@media (max-width: 620px) {
  .wizard-shell .topbar {
    padding: 0 16px;
  }

  .wizard-shell .topbar-caption {
    font-size: 8px;
  }

  .app-shell.wizard-shell .wizard-workspace {
    width: calc(100% - 24px);
    height: calc(100vh - 66px);
    grid-template-rows: 44px minmax(0, 1fr) 56px;
    gap: 10px;
    padding: 14px 0 12px;
  }

  .stepper {
    gap: 5px;
  }

  .step-item {
    justify-content: center;
    gap: 5px;
    padding: 0 4px;
  }

  .step-number {
    width: 22px;
    height: 22px;
    flex-basis: 22px;
    font-size: 9px;
  }

  .step-label {
    font-size: 9px;
  }

  .step-heading {
    margin-bottom: 13px;
  }

  .step-heading h1 {
    font-size: 21px;
  }

  .step-description {
    font-size: 11px;
  }

  .step-surface {
    padding: 13px;
  }

  .requirement-status {
    font-size: 9px;
  }

  .wizard-page .actions {
    grid-template-columns: 1fr;
    gap: 7px;
  }

  .wizard-page .action-button {
    min-height: 48px;
  }

  .back-button,
  .continue-button {
    min-width: 88px;
    padding: 0 10px;
  }
}

@media (max-height: 650px) and (min-width: 621px) {
  .wizard-workspace {
    padding-top: 12px;
    padding-bottom: 10px;
    gap: 9px;
  }

  .wizard-page .step-heading {
    margin-bottom: 10px;
  }

  .step-surface {
    padding: 12px;
  }
}
</style>

<style scoped>
:global(:root) {
  color-scheme: dark;
  --page: #101512;
  --surface: #171e1a;
  --surface-raised: #1d2721;
  --surface-hover: #26332b;
  --line: #303d34;
  --line-strong: #425146;
  --text: #e8eee8;
  --muted: #9aa79b;
  --accent: #c9ef77;
  --success: #a9d274;
  --warning: #e9a47e;
}

:global(body) {
  background: var(--page);
  color: var(--text);
}

:global(button:focus-visible),
:global(a:focus-visible) {
  outline: 2px solid var(--accent);
  outline-offset: 3px;
}

.app-shell {
  background-color: var(--page);
  background-image:
    linear-gradient(rgba(201, 239, 119, 0.025) 1px, transparent 1px),
    linear-gradient(90deg, rgba(201, 239, 119, 0.025) 1px, transparent 1px);
  background-size: 36px 36px;
}

.topbar {
  border-color: var(--line);
  background: rgba(21, 28, 24, 0.96);
}

.brand-name,
.page-heading h1,
.dependency-heading h2,
.plugin-manager-heading h2 {
  color: var(--text);
}

.topbar-caption,
.distribution-name,
.connection-state,
.dependency-message,
.manager-note {
  color: var(--muted);
}

.workspace {
  width: min(100% - 40px, 920px);
  padding-top: 56px;
}

.page-heading {
  margin-bottom: 25px;
}

.page-heading .eyebrow,
.dependency-panel .eyebrow,
.plugin-manager .eyebrow {
  color: #a8c879;
}

.connection-state {
  padding: 8px 11px;
  border: 1px solid var(--line);
  border-radius: 999px;
  background: var(--surface);
}

.state-dot {
  box-shadow: 0 0 10px rgba(169, 210, 116, 0.35);
}

.state-dot.busy {
  box-shadow: 0 0 10px rgba(233, 164, 126, 0.4);
}

.actions {
  gap: 14px;
  margin-bottom: 22px;
}

.action-button {
  min-height: 68px;
  border-color: var(--line);
  border-radius: 7px;
  gap: 15px;
  padding: 0 20px;
  font-size: 14px;
}

.status-button {
  background: #243a2d;
  color: #eff8e9;
}

.sync-button {
  background: #633b2c;
  color: #fff1e9;
}

.action-button:hover:not(:disabled) {
  border-color: var(--line-strong);
  filter: brightness(1.16);
}

.button-indicator {
  color: var(--accent);
}

.sync-button .button-indicator {
  color: #ffc09d;
}

.dependency-panel,
.plugin-manager {
  margin-bottom: 18px;
  padding: 20px 21px;
  border: 1px solid var(--line);
  border-radius: 7px;
  background: rgba(23, 30, 26, 0.96);
  box-shadow: 0 12px 30px rgba(0, 0, 0, 0.14);
}

.dependency-heading,
.plugin-manager-heading {
  margin-bottom: 16px;
}

.dependency-heading h2,
.plugin-manager-heading h2 {
  font-size: 16px;
}

.dependency-item {
  border-color: var(--line);
  border-radius: 5px;
  background: var(--surface-raised);
  color: var(--text);
}

.dependency-state {
  color: var(--success);
}

.dependency-state.missing {
  color: var(--warning);
}

.all-installed {
  color: var(--success);
}

.refresh-button,
.cancel-button {
  color: #c5d0c4;
}

.refresh-button:hover:not(:disabled),
.cancel-button:hover {
  background: var(--surface-hover);
  color: var(--text);
}

.path-label {
  color: #c5d0c4;
}

.path-control input {
  border-color: var(--line-strong);
  background: #101612;
  color: var(--text);
}

.path-control input::placeholder {
  color: #738075;
}

.path-control input:focus-visible {
  outline-color: var(--accent);
}

.browse-button,
.remove-location-button {
  border-color: var(--line-strong);
  background: var(--surface-raised);
  color: #d7e0d7;
}

.browse-button:hover:not(:disabled),
.remove-location-button:hover:not(:disabled) {
  border-color: #697b6c;
  background: var(--surface-hover);
}

.install-button {
  border-color: #6b8748;
  background: #476032;
  color: #f3fae7;
}

.install-button:hover:not(:disabled) {
  border-color: #8eaa61;
  background: #58783b;
}

.documentation-link {
  color: #c2dc91;
}

.install-confirmation {
  border-left-color: #d88c66;
  background: #2a211d;
}

.install-confirmation p {
  color: #c5b7af;
}

.terminal {
  border-color: var(--line);
  border-radius: 7px;
  background: #0c110e;
  box-shadow: 0 16px 36px rgba(0, 0, 0, 0.24);
}

.terminal-header {
  border-color: var(--line);
  background: #171f1a;
}

.terminal-header h2 {
  color: #c6d0c7;
}

.terminal-command {
  color: #90a08f;
}

.terminal-output,
.installation-output,
.locations-output {
  background: #0c110e;
  color: #cce8b0;
}

@media (max-width: 560px) {
  .workspace {
    width: calc(100% - 28px);
    padding-top: 38px;
  }

  .page-heading {
    align-items: flex-start;
  }

  .connection-state {
    padding: 6px 8px;
    font-size: 10px;
  }

  .action-button {
    min-height: 60px;
  }

  .dependency-panel,
  .plugin-manager {
    padding: 16px;
  }
}
</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>

<style scoped>
:global(*) {
  box-sizing: border-box;
}

:global(body) {
  margin: 0;
  min-height: 100vh;
  background: #f0f2ed;
  color: #202820;
  font-family: "Avenir Next", "Segoe UI", sans-serif;
}

:global(button) {
  font: inherit;
}

.app-shell {
  min-height: 100vh;
  background-image: linear-gradient(rgba(32, 40, 32, 0.025) 1px, transparent 1px);
  background-size: 100% 32px;
}

.topbar {
  height: 66px;
  display: flex;
  align-items: center;
  gap: 11px;
  padding: 0 7.5%;
  border-bottom: 1px solid #d9ded6;
  background: rgba(248, 249, 246, 0.82);
}

.brand-mark {
  display: grid;
  place-items: center;
  width: 27px;
  height: 27px;
  background: #c9ef77;
  color: #24311e;
  font-size: 15px;
  font-weight: 800;
}

.brand-name {
  font-size: 14px;
  font-weight: 700;
}

.topbar-caption {
  margin-left: auto;
  color: #7a8378;
  font-size: 10px;
  font-weight: 700;
  letter-spacing: 1.2px;
}

.workspace {
  width: min(100% - 48px, 860px);
  margin: 0 auto;
  padding: 74px 0 90px;
}

.page-heading {
  display: flex;
  align-items: end;
  justify-content: space-between;
  gap: 20px;
  margin-bottom: 35px;
}

.eyebrow {
  margin: 0 0 11px;
  color: #668140;
  font-size: 10px;
  font-weight: 800;
  letter-spacing: 1.5px;
}

h1 {
  margin: 0;
  text-align: left;
  font-size: clamp(26px, 4vw, 36px);
  font-weight: 650;
  line-height: 1.15;
  letter-spacing: 0;
}

.connection-state {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding-bottom: 4px;
  color: #596258;
  font-size: 12px;
}

.state-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #7b9f4b;
}

.state-dot.busy {
  background: #dc9851;
}

.dependency-panel {
  margin-bottom: 30px;
  padding: 20px;
  border: 1px solid #d5dad1;
  border-radius: 5px;
  background: rgba(248, 249, 246, 0.88);
}

.dependency-heading {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  margin-bottom: 17px;
}

.dependency-heading .eyebrow {
  margin-bottom: 6px;
}

.dependency-heading h2 {
  margin: 0;
  font-size: 17px;
  font-weight: 650;
}

.refresh-button,
.cancel-button {
  border: 0;
  border-radius: 3px;
  box-shadow: none;
  background: transparent;
  color: #53654c;
  font-size: 11px;
  font-weight: 650;
  cursor: pointer;
}

.refresh-button:hover:not(:disabled),
.cancel-button:hover {
  border-color: transparent;
  background: #e8ede2;
}

.refresh-button:disabled {
  cursor: wait;
  opacity: 0.65;
}

.distribution-name {
  margin: 0 0 12px;
  color: #657065;
  font-size: 12px;
}

.dependency-list {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 10px;
  margin-bottom: 15px;
}

.dependency-item {
  min-height: 45px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 10px;
  padding: 0 12px;
  border: 1px solid #e0e4dd;
  border-radius: 3px;
  background: #fff;
  font-size: 12px;
  font-weight: 600;
}

.dependency-state {
  color: #4c753e;
  font-size: 10px;
  white-space: nowrap;
}

.dependency-state.missing {
  color: #a44f35;
}

.dependency-message {
  margin: 0;
  color: #626c61;
  font-size: 12px;
  line-height: 1.65;
}

.install-actions,
.confirmation-actions {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 14px;
  margin-top: 16px;
}

.install-button {
  min-height: 38px;
  padding: 0 15px;
  border: 1px solid #354c3c;
  border-radius: 4px;
  box-shadow: none;
  background: #354c3c;
  color: #f8faf5;
  font-size: 12px;
  font-weight: 650;
  cursor: pointer;
}

.install-button:hover:not(:disabled) {
  border-color: #354c3c;
  background: #45634e;
}

.install-button:disabled {
  cursor: wait;
  opacity: 0.7;
}

.documentation-link {
  color: #6a7d4e;
  font-size: 11px;
  font-weight: 650;
  text-decoration: underline;
  text-underline-offset: 3px;
}

.install-confirmation {
  margin-top: 17px;
  padding: 17px;
  border-left: 3px solid #a44f35;
  background: #f4eee8;
}

.install-confirmation h3 {
  margin: 0 0 9px;
  font-size: 14px;
}

.install-confirmation p {
  margin: 7px 0;
  color: #5e6259;
  font-size: 11px;
  line-height: 1.6;
}

.cancel-button {
  min-height: 36px;
  padding: 0 12px;
}

.installation-output {
  max-height: 160px;
  overflow: auto;
  margin: 16px 0 0;
  padding: 12px;
  background: #202b24;
  color: #d0e7bb;
  font-family: "SFMono-Regular", Consolas, monospace;
  font-size: 10px;
  line-height: 1.6;
  white-space: pre-wrap;
  overflow-wrap: anywhere;
}

.all-installed {
  margin: 0;
  color: #4c753e;
  font-size: 12px;
}

.plugin-manager {
  margin-bottom: 30px;
  padding: 20px;
  border: 1px solid #d5dad1;
  border-radius: 5px;
  background: rgba(248, 249, 246, 0.88);
}

.plugin-manager-heading {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  margin-bottom: 18px;
}

.plugin-manager-heading .eyebrow {
  margin-bottom: 6px;
}

.plugin-manager-heading h2 {
  margin: 0;
  font-size: 17px;
  font-weight: 650;
}

.path-label {
  display: block;
  margin-bottom: 7px;
  color: #586357;
  font-size: 11px;
  font-weight: 650;
}

.path-control {
  display: flex;
  gap: 8px;
}

.path-control input {
  min-width: 0;
  flex: 1;
  min-height: 40px;
  padding: 0 11px;
  border: 1px solid #cbd2c8;
  border-radius: 4px;
  box-shadow: none;
  background: #fff;
  color: #293229;
  font-family: "SFMono-Regular", Consolas, monospace;
  font-size: 11px;
}

.path-control input:focus-visible {
  outline: 2px solid #81976a;
  outline-offset: 1px;
}

.browse-button,
.remove-location-button {
  min-height: 38px;
  padding: 0 12px;
  border: 1px solid #cbd2c8;
  border-radius: 4px;
  box-shadow: none;
  background: #f7f8f5;
  color: #3c493a;
  font-size: 11px;
  font-weight: 650;
  cursor: pointer;
}

.browse-button:hover:not(:disabled),
.remove-location-button:hover:not(:disabled) {
  border-color: #9da997;
  background: #eef1eb;
}

.location-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 9px;
  margin-top: 10px;
}

.location-actions .install-button,
.location-actions .remove-location-button {
  min-height: 36px;
}

.location-actions .install-button {
  font-size: 11px;
}

.browse-button:disabled,
.remove-location-button:disabled {
  cursor: not-allowed;
  opacity: 0.6;
}

.manager-note {
  margin: 12px 0 0;
  color: #687267;
  font-size: 10px;
  line-height: 1.6;
}

.locations-output {
  max-height: 200px;
  overflow: auto;
  margin: 12px 0 0;
  padding: 13px;
  border-radius: 3px;
  background: #202b24;
  color: #d0e7bb;
  font-family: "SFMono-Regular", Consolas, monospace;
  font-size: 10px;
  line-height: 1.65;
  white-space: pre-wrap;
  overflow-wrap: anywhere;
}

.actions {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 12px;
  margin-bottom: 34px;
}

.action-button {
  min-height: 76px;
  display: flex;
  align-items: center;
  gap: 14px;
  padding: 0 20px;
  border: 1px solid transparent;
  border-radius: 5px;
  box-shadow: none;
  color: #f8faf5;
  text-align: left;
  font-size: 14px;
  font-weight: 650;
  cursor: pointer;
  transition: transform 140ms ease, filter 140ms ease;
}

.action-button:hover:not(:disabled) {
  border-color: transparent;
  filter: brightness(1.08);
  transform: translateY(-1px);
}

.action-button:active {
  background-color: inherit;
}

.action-button:focus-visible {
  outline: 3px solid #c9ef77;
  outline-offset: 3px;
}

.action-button:disabled {
  cursor: wait;
  opacity: 0.72;
}

.status-button {
  background: #354c3c;
}

.sync-button {
  background: #a44f35;
}

.button-indicator {
  color: #c9ef77;
  font-family: "SFMono-Regular", Consolas, monospace;
  font-size: 11px;
}

.sync-note {
  color: rgba(255, 255, 255, 0.72);
  font-weight: 450;
}

.button-arrow {
  margin-left: auto;
  font-size: 18px;
  font-weight: 400;
}

.terminal {
  overflow: hidden;
  border: 1px solid #29342d;
  border-radius: 5px;
  background: #18211c;
  box-shadow: 0 16px 35px rgba(39, 49, 40, 0.12);
}

.terminal-header {
  min-height: 43px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 0 16px;
  border-bottom: 1px solid #303d35;
  background: #202b24;
}

.terminal-title-group,
.terminal-lights {
  display: flex;
  align-items: center;
}

.terminal-title-group {
  gap: 13px;
}

.terminal-lights {
  gap: 5px;
}

.terminal-lights i {
  width: 7px;
  height: 7px;
  border-radius: 50%;
  background: #c36c51;
}

.terminal-lights i:nth-child(2) {
  background: #d3a24f;
}

.terminal-lights i:nth-child(3) {
  background: #83a65a;
}

.terminal-header h2 {
  margin: 0;
  color: #c5cec4;
  font-size: 9px;
  font-weight: 700;
  letter-spacing: 1px;
}

.terminal-command {
  overflow: hidden;
  color: #869388;
  font-family: "SFMono-Regular", Consolas, monospace;
  font-size: 10px;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.terminal-output {
  min-height: 250px;
  max-height: 430px;
  overflow: auto;
  margin: 0;
  padding: 22px 20px;
  color: #d0e7bb;
  font-family: "SFMono-Regular", Consolas, "Liberation Mono", monospace;
  font-size: 12px;
  line-height: 1.75;
  white-space: pre-wrap;
  overflow-wrap: anywhere;
}

@media (max-width: 560px) {
  .topbar {
    padding: 0 20px;
  }

  .workspace {
    width: calc(100% - 36px);
    padding-top: 48px;
  }

  .actions {
    grid-template-columns: 1fr;
    gap: 9px;
  }

  .dependency-panel {
    padding: 16px;
  }

  .plugin-manager {
    padding: 16px;
  }

  .path-control {
    flex-direction: column;
  }

  .browse-button {
    min-height: 38px;
  }

  .dependency-list {
    grid-template-columns: 1fr;
  }

  .dependency-item {
    font-size: 11px;
  }

  .action-button {
    min-height: 64px;
  }

  .terminal-output {
    min-height: 220px;
    padding: 17px 15px;
    font-size: 11px;
  }
}
</style>

<style scoped>
:global(html body) {
  background: var(--page);
  color: var(--text);
}

.app-shell {
  color: var(--text);
}

.app-shell .topbar {
  border-color: var(--line);
  background: rgba(21, 28, 24, 0.96);
}

.app-shell .settings-trigger {
  min-height: 34px;
  display: inline-flex;
  align-items: center;
  gap: 8px;
  margin-left: 12px;
  padding: 0 11px;
  border: 1px solid #3b493e;
  border-radius: 5px;
  box-shadow: none;
  background: #1d2721;
  color: #d7e0d7;
  font: inherit;
  font-size: 10px;
  font-weight: 700;
  cursor: pointer;
}

.app-shell .settings-trigger:hover {
  border-color: #687f4b;
  background: #26332b;
}

.settings-icon {
  color: #c9ef77;
  font-size: 14px;
  line-height: 1;
}

.app-shell .wizard-workspace.settings-mode {
  grid-template-rows: minmax(0, 1fr);
}

.settings-page {
  width: min(100%, 600px);
  height: 100%;
  min-height: 0;
  overflow: auto;
  margin: 0 auto;
  padding: 20px 4px;
}

.settings-page .step-heading {
  margin-bottom: 18px;
}

.settings-card {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 20px;
  margin-bottom: 12px;
  padding: 18px;
  border: 1px solid #303d34;
  border-radius: 7px;
  background: #171e1a;
}

.settings-card h2 {
  margin: 0;
  color: #e8eee8;
  font-size: 14px;
  font-weight: 650;
}

.settings-card-eyebrow {
  margin: 0 0 6px;
  color: #a8c879;
  font-size: 8px;
  font-weight: 750;
  letter-spacing: 1px;
}

.language-field {
  min-width: 190px;
  display: flex;
  flex-direction: column;
  gap: 6px;
  color: #9aa79b;
  font-size: 10px;
}

.language-field select {
  min-height: 38px;
  padding: 0 10px;
  border: 1px solid #425146;
  border-radius: 4px;
  background: #101612;
  color: #e8eee8;
  font: inherit;
  font-size: 11px;
}

.language-field select:focus-visible {
  outline: 2px solid #c9ef77;
  outline-offset: 2px;
}

.language-field option {
  background: #171e1a;
  color: #e8eee8;
}

.about-card {
  align-items: center;
}

.about-description {
  max-width: 380px;
  margin: 7px 0 0;
  color: #9aa79b;
  font-size: 10px;
  line-height: 1.55;
}

.version-badge {
  min-width: 90px;
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 10px 12px;
  border: 1px solid #303d34;
  border-radius: 5px;
  background: #1c241f;
  text-align: right;
}

.version-badge span {
  color: #819084;
  font-size: 9px;
}

.version-badge strong {
  color: #c9ef77;
  font-family: "SFMono-Regular", Consolas, monospace;
  font-size: 12px;
}

.app-shell .workspace {
  width: min(100% - 40px, 920px);
  padding-top: 56px;
}

.app-shell .page-heading h1,
.app-shell .dependency-heading h2,
.app-shell .plugin-manager-heading h2 {
  color: var(--text);
}

.app-shell .topbar-caption,
.app-shell .connection-state,
.app-shell .distribution-name,
.app-shell .dependency-message,
.app-shell .manager-note {
  color: var(--muted);
}

.app-shell .page-heading .eyebrow,
.app-shell .dependency-panel .eyebrow,
.app-shell .plugin-manager .eyebrow {
  color: #a8c879;
}

.app-shell .actions {
  gap: 14px;
  margin-bottom: 22px;
}

.app-shell .action-button {
  min-height: 68px;
  border-color: var(--line);
  border-radius: 7px;
  gap: 15px;
  padding: 0 20px;
  font-size: 14px;
}

.app-shell .status-button {
  background: #243a2d;
  color: #eff8e9;
}

.app-shell .sync-button {
  background: #633b2c;
  color: #fff1e9;
}

.app-shell .action-button:hover:not(:disabled) {
  border-color: var(--line-strong);
  filter: brightness(1.16);
}

.app-shell .button-indicator {
  color: var(--accent);
}

.app-shell .sync-button .button-indicator {
  color: #ffc09d;
}

.app-shell .dependency-panel,
.app-shell .plugin-manager {
  margin-bottom: 18px;
  padding: 20px 21px;
  border: 1px solid var(--line);
  border-radius: 7px;
  background: rgba(23, 30, 26, 0.96);
  box-shadow: 0 12px 30px rgba(0, 0, 0, 0.14);
}

.app-shell .dependency-item {
  border-color: var(--line);
  border-radius: 5px;
  background: var(--surface-raised);
  color: var(--text);
}

.app-shell .dependency-state {
  color: var(--success);
}

.app-shell .dependency-state.missing {
  color: var(--warning);
}

.app-shell .all-installed {
  color: var(--success);
}

.app-shell .refresh-button,
.app-shell .cancel-button {
  color: #c5d0c4;
}

.app-shell .refresh-button:hover:not(:disabled),
.app-shell .cancel-button:hover {
  background: var(--surface-hover);
  color: var(--text);
}

.app-shell .path-label {
  color: #c5d0c4;
}

.app-shell .path-control input {
  border-color: var(--line-strong);
  background: #101612;
  color: var(--text);
}

.app-shell .path-control input::placeholder {
  color: #738075;
}

.app-shell .browse-button,
.app-shell .remove-location-button {
  border-color: var(--line-strong);
  background: var(--surface-raised);
  color: #d7e0d7;
}

.app-shell .browse-button:hover:not(:disabled),
.app-shell .remove-location-button:hover:not(:disabled) {
  border-color: #697b6c;
  background: var(--surface-hover);
}

.app-shell .install-button {
  border-color: #6b8748;
  background: #476032;
  color: #f3fae7;
}

.app-shell .install-button:hover:not(:disabled) {
  border-color: #8eaa61;
  background: #58783b;
}

.app-shell .documentation-link {
  color: #c2dc91;
}

.app-shell .install-confirmation {
  border-left-color: #d88c66;
  background: #2a211d;
}

.app-shell .install-confirmation p {
  color: #c5b7af;
}

.app-shell .terminal {
  border-color: var(--line);
  border-radius: 7px;
  background: #0c110e;
  box-shadow: 0 16px 36px rgba(0, 0, 0, 0.24);
}

.app-shell .terminal-header {
  border-color: var(--line);
  background: #171f1a;
}

.app-shell .terminal-header h2 {
  color: #c6d0c7;
}

.app-shell .terminal-command {
  color: #90a08f;
}

.app-shell .terminal-output,
.app-shell .installation-output,
.app-shell .locations-output {
  background: #0c110e;
  color: #cce8b0;
}

@media (max-width: 560px) {
  .app-shell .settings-trigger {
    gap: 5px;
    margin-left: 6px;
    padding: 0 7px;
    font-size: 9px;
  }

  .settings-card {
    align-items: stretch;
    flex-direction: column;
    gap: 14px;
    padding: 15px;
  }

  .language-field {
    min-width: 0;
  }

  .version-badge {
    align-self: flex-start;
    min-width: 100%;
    text-align: left;
  }

  .app-shell .workspace {
    width: calc(100% - 28px);
    padding-top: 38px;
  }

  .app-shell .page-heading {
    align-items: flex-start;
  }

  .app-shell .connection-state {
    padding: 6px 8px;
    font-size: 10px;
  }

  .app-shell .action-button {
    min-height: 60px;
  }

  .app-shell .dependency-panel,
  .app-shell .plugin-manager {
    padding: 16px;
  }
}
</style>