import type { Meta, StoryObj } from '@storybook/vue3';

import Toaster from '../components/ui/toast/Toaster.vue';

const meta = {
  title: 'Toaster',
  component: Toaster,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof Toaster>;

export default meta;
type Story = StoryObj<typeof Toaster>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};