// stores/counter.js
import { defineStore } from 'pinia'
import { ref, type Ref } from 'vue'

export const useUtilsStore = defineStore('utils', () => {

    const toast = ref({
        content: '',
        isSuccess: false,
        isVisible: false,
    });

    const showToast = (error: string, isSuccess: boolean, timeout?: number ) => {
        toast.value.isVisible = true;
        toast.value.isSuccess = isSuccess;
        toast.value.content = error;
        setTimeout(() => {
            toast.value.isVisible = false;
        }, timeout || 3000);
    }

    return {toast, showToast}

})
