import type { Meta, StoryObj } from '@storybook/vue3';

import MenubarTrigger from '../components/ui/menubar/MenubarTrigger.vue';

const meta = {
  title: 'MenubarTrigger',
  component: MenubarTrigger,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof MenubarTrigger>;

export default meta;
type Story = StoryObj<typeof MenubarTrigger>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};