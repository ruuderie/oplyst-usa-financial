import type { Meta, StoryObj } from '@storybook/vue3';

import AlertDialogTrigger from '../components/ui/alert-dialog/AlertDialogTrigger.vue';

const meta = {
  title: 'AlertDialogTrigger',
  component: AlertDialogTrigger,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof AlertDialogTrigger>;

export default meta;
type Story = StoryObj<typeof AlertDialogTrigger>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};