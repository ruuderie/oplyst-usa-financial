import type { Meta, StoryObj } from '@storybook/vue3';

import DialogTitle from '../components/ui/dialog/DialogTitle.vue';

const meta = {
  title: 'DialogTitle',
  component: DialogTitle,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof DialogTitle>;

export default meta;
type Story = StoryObj<typeof DialogTitle>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};