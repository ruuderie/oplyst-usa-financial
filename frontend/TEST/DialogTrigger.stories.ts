import type { Meta, StoryObj } from '@storybook/vue3';

import DialogTrigger from '../components/ui/dialog/DialogTrigger.vue';

const meta = {
  title: 'DialogTrigger',
  component: DialogTrigger,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof DialogTrigger>;

export default meta;
type Story = StoryObj<typeof DialogTrigger>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};