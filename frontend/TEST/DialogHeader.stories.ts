import type { Meta, StoryObj } from '@storybook/vue3';

import DialogHeader from '../components/ui/dialog/DialogHeader.vue';

const meta = {
  title: 'DialogHeader',
  component: DialogHeader,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof DialogHeader>;

export default meta;
type Story = StoryObj<typeof DialogHeader>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};