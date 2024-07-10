import type { Meta, StoryObj } from '@storybook/vue3';

import AlertDialogFooter from '../components/ui/alert-dialog/AlertDialogFooter.vue';

const meta = {
  title: 'AlertDialogFooter',
  component: AlertDialogFooter,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof AlertDialogFooter>;

export default meta;
type Story = StoryObj<typeof AlertDialogFooter>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};